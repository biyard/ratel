#[cfg(feature = "web")]
use crate::common::{
    controllers::AssetPresignedUris, wasm_bindgen, wasm_bindgen_futures, web_sys, Error,
};
use crate::*;
use dioxus::html::FileData;
use dioxus::html::HasFileData;

#[derive(Clone, Debug, PartialEq)]
pub struct UploadedFileMeta {
    pub url: String,
    pub name: String,
    pub size: String,
}

#[component]
pub fn FileUploader(
    on_upload_success: EventHandler<String>,
    #[props(default)] on_upload_meta: Option<EventHandler<UploadedFileMeta>>,
    class: Option<String>,
    accept: Option<String>,
    children: Element,
) -> Element {
    let accept = accept.unwrap_or_else(|| "image/*".to_string());
    let class_name = if let Some(class) = class {
        format!("cursor-pointer {}", class)
    } else {
        "cursor-pointer".to_string()
    };

    // Incrementing this key forces the file input element to be recreated,
    // resetting its value so the same file can be re-selected after deletion.
    let mut input_key = use_signal(|| 0u32);

    let start_upload = {
        let accept = accept.clone();
        let on_upload_success = on_upload_success.clone();
        let on_upload_meta = on_upload_meta.clone();
        move |file: FileData| {
            let accept = accept.clone();
            let on_upload_success = on_upload_success.clone();
            let on_upload_meta = on_upload_meta.clone();
            spawn(async move {
                if let Err(err) =
                    upload_via_presigned(&accept, file, on_upload_success, on_upload_meta).await
                {
                    error!("FileUploader: {}", &err.to_string());
                }
            });
        }
    };

    let on_change = {
        let start_upload = start_upload.clone();
        move |evt: FormEvent| {
            let Some(file) = evt.files().into_iter().next() else {
                return;
            };
            start_upload(file);
            // Reset the file input so the same file can be uploaded again
            input_key += 1;
        }
    };

    let on_drop = {
        let start_upload = start_upload.clone();
        move |evt: DragEvent| {
            evt.prevent_default();
            let Some(file) = evt.files().into_iter().next() else {
                return;
            };
            start_upload(file);
        }
    };

    let on_drag_over = move |evt: DragEvent| {
        evt.prevent_default();
    };

    rsx! {
        label {
            class: "{class_name}",
            ondragover: on_drag_over,
            ondrop: on_drop,
            input {
                key: "{input_key}",
                class: "hidden",
                r#type: "file",
                accept: "{accept}",
                onchange: on_change,
            }
            {children}
        }
    }
}

#[cfg(not(feature = "web"))]
async fn upload_via_presigned(
    _accept: &str,
    _file: FileData,
    _on_upload_success: EventHandler<String>,
    _on_upload_meta: Option<EventHandler<UploadedFileMeta>>,
) -> Result<()> {
    Ok(())
}

#[cfg(feature = "web")]
async fn upload_via_presigned(
    _accept: &str,
    file: FileData,
    on_upload_success: EventHandler<String>,
    on_upload_meta: Option<EventHandler<UploadedFileMeta>>,
) -> Result<()> {
    use dioxus::web::WebFileExt;
    use wasm_bindgen::JsCast;

    let file_name = file.name();
    let Some(web_file) = file.get_web_file() else {
        return Err(Error::NotFound("Failed to get web file".to_string()));
    };

    // FIXME: use mimetype_detector
    // if !is_allowed_file(accept, &file) {
    //     return Err(Error::NotSupported("Not supported files.".to_string()));
    // }

    if web_file.size() > 100_f64 * 1024_f64 * 1024_f64 {
        return Err(Error::NotSupported(
            "Files larger than 100MB are not supported.".to_string(),
        ));
    }

    let file_type = guess_file_type(&file, &web_file);
    let presigned = request_presigned_url(&file_type).await?;
    let presigned_url = presigned
        .presigned_uris
        .get(0)
        .ok_or_else(|| Error::NotFound("Missing presigned URL.".to_string()))?
        .to_string();
    let public_url = presigned
        .uris
        .get(0)
        .ok_or_else(|| Error::NotFound("Missing public URL.".to_string()))?
        .to_string();

    let content_type = web_file.type_();
    let size = format_file_size(web_file.size());
    let opts = web_sys::RequestInit::new();
    opts.set_method("PUT");
    let body = wasm_bindgen::JsValue::from(web_file);
    opts.set_body(&body);

    let request = web_sys::Request::new_with_str_and_init(&presigned_url, &opts)
        .map_err(|e| Error::Unknown(js_error_to_string(e)))?;
    if !content_type.is_empty() {
        request
            .headers()
            .set("Content-Type", &content_type)
            .map_err(|e| Error::Unknown(js_error_to_string(e)))?;
    }

    let window =
        web_sys::window().ok_or_else(|| Error::NotFound("No window available.".to_string()))?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| Error::Unknown(js_error_to_string(e)))?;
    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|_| Error::Unknown("Invalid upload response.".to_string()))?;
    if !resp.ok() {
        return Err(Error::InternalServerError(format!(
            "Upload failed ({})",
            resp.status()
        )));
    }

    on_upload_success.call(public_url.clone());
    if let Some(on_upload_meta) = on_upload_meta {
        on_upload_meta.call(UploadedFileMeta {
            url: public_url,
            name: file_name,
            size,
        });
    }
    Ok(())
}

// #[cfg(feature = "web")]
// fn is_allowed_file(accept: &str, file: &FileData) -> bool {
//     let name = file.name().to_lowercase();
//     let ext = name.rsplit('.').next().unwrap_or("");
//     accept
//         .split(',')
//         .map(|item| item.trim().trim_start_matches('.').to_lowercase())
//         .any(|allowed| !allowed.is_empty() && allowed == ext)
// }

#[cfg(feature = "web")]
fn format_file_size(size_bytes: f64) -> String {
    let mb = size_bytes / (1024_f64 * 1024_f64);
    if mb >= 1_f64 {
        format!("{:.1} MB", mb)
    } else {
        let kb = size_bytes / 1024_f64;
        format!("{:.1} KB", kb)
    }
}

#[cfg(feature = "web")]
async fn request_presigned_url(file_type: &str) -> Result<AssetPresignedUris> {
    crate::common::controllers::get_put_object_uri(Some(1), Some(file_type.to_string())).await
}

#[cfg(feature = "web")]
fn guess_file_type(file: &FileData, web_file: &web_sys::File) -> String {
    let name = file.name().to_lowercase();
    if let Some(ext) = name.split('.').last() {
        if ext != name {
            return ext.to_string();
        }
    }
    let mime = web_file.type_().to_lowercase();
    if mime.contains("png") {
        "png".to_string()
    } else if mime.contains("jpeg") || mime.contains("jpg") {
        "jpg".to_string()
    } else if mime.contains("pdf") {
        "pdf".to_string()
    } else {
        "bin".to_string()
    }
}

#[cfg(feature = "web")]
fn web_log_error(message: &str) {
    web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(message));
}

#[cfg(feature = "web")]
fn js_error_to_string(err: wasm_bindgen::JsValue) -> String {
    if let Some(msg) = err.as_string() {
        msg
    } else {
        "Unknown error".to_string()
    }
}
