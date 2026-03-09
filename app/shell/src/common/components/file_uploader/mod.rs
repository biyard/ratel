#[cfg(feature = "web")]
use crate::common::{
    Error, controllers::AssetPresignedUris, wasm_bindgen, wasm_bindgen_futures, web_sys,
};
#[cfg(feature = "web")]
use dioxus::html::FileData;
use dioxus::prelude::*;

#[cfg(feature = "web")]
type UploadResult<T> = crate::common::Result<T>;

#[component]
pub fn FileUploader(
    on_upload_success: EventHandler<String>,
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

    #[cfg(feature = "web")]
    let on_change = {
        let accept = accept.clone();
        let on_upload_success = on_upload_success.clone();
        move |evt: FormEvent| {
            let Some(file) = evt.files().into_iter().next() else {
                return;
            };
            let accept = accept.clone();
            let on_upload_success = on_upload_success.clone();
            spawn(async move {
                if let Err(err) =
                    upload_via_presigned(&accept, file, on_upload_success).await
                {
                    web_log_error(&err.to_string());
                }
            });
        }
    };

    #[cfg(not(feature = "web"))]
    let on_change = |_evt: FormEvent| {};

    rsx! {
        label { class: "{class_name}",
            input {
                class: "hidden",
                r#type: "file",
                accept: "{accept}",
                onchange: on_change,
            }
            {children}
        }
    }
}

#[cfg(feature = "web")]
async fn upload_via_presigned(
    accept: &str,
    file: FileData,
    on_upload_success: EventHandler<String>,
) -> UploadResult<()> {
    use dioxus::web::WebFileExt;
    use wasm_bindgen::JsCast;

    let Some(web_file) = file.get_web_file() else {
        return Err(Error::NotFound("Failed to get web file".to_string()));
    };

    if accept.contains("image") && !web_file.type_().starts_with("image/") {
        return Err(Error::NotSupported(
            "Only image types are supported.".to_string(),
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

    on_upload_success.call(public_url);
    Ok(())
}

#[cfg(feature = "web")]
async fn request_presigned_url(file_type: &str) -> UploadResult<AssetPresignedUris> {
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
