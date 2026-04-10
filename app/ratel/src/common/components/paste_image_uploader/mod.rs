use crate::common::components::image_upload_preview::PendingImage;
use dioxus::prelude::*;

pub const MAX_COMMENT_IMAGES: usize = 4;
pub const MAX_IMAGE_SIZE_BYTES: f64 = 10.0 * 1024.0 * 1024.0; // 10MB

/// Call from an onpaste handler. Returns true if an image was captured
/// (caller should preventDefault), false for normal text paste.
#[cfg(feature = "web")]
pub fn handle_paste_event(
    event: &ClipboardEvent,
    mut pending_images: Signal<Vec<PendingImage>>,
) -> bool {
    use dioxus::web::WebEventExt;
    use wasm_bindgen::JsCast;
    use web_sys::js_sys::Reflect;

    let Some(web_event) = event.try_as_web_event() else {
        return false;
    };
    let js_event: &wasm_bindgen::JsValue = web_event.unchecked_ref();

    // Access clipboardData
    let clipboard_data = match Reflect::get(js_event, &wasm_bindgen::JsValue::from_str("clipboardData")) {
        Ok(v) if !v.is_null() && !v.is_undefined() => v,
        _ => return false,
    };

    // Access clipboardData.files
    let files = match Reflect::get(&clipboard_data, &wasm_bindgen::JsValue::from_str("files")) {
        Ok(v) if !v.is_null() && !v.is_undefined() => v,
        _ => return false,
    };

    // Access files.length
    let length = match Reflect::get(&files, &wasm_bindgen::JsValue::from_str("length")) {
        Ok(v) => v.as_f64().unwrap_or(0.0) as u32,
        _ => return false,
    };
    if length == 0 {
        return false;
    }

    // Access files[0]
    let file_js = match Reflect::get_u32(&files, 0) {
        Ok(v) if !v.is_null() && !v.is_undefined() => v,
        _ => return false,
    };
    let Ok(file) = file_js.dyn_into::<web_sys::File>() else {
        return false;
    };

    // Must be an image
    let file_type = file.type_();
    if !file_type.starts_with("image/") {
        return false;
    }

    // Max count check
    if pending_images.read().len() >= MAX_COMMENT_IMAGES {
        tracing::warn!("Maximum {} images per comment", MAX_COMMENT_IMAGES);
        return true;
    }

    // Size check
    if file.size() > MAX_IMAGE_SIZE_BYTES {
        tracing::warn!(
            "Image too large: {} bytes (max {})",
            file.size(),
            MAX_IMAGE_SIZE_BYTES
        );
        return true;
    }

    // Create local preview via URL.createObjectURL
    let local_url = {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return true,
        };
        let url_obj =
            match Reflect::get(&window, &wasm_bindgen::JsValue::from_str("URL")) {
                Ok(v) if !v.is_null() && !v.is_undefined() => v,
                _ => return true,
            };
        let create_fn = match Reflect::get(
            &url_obj,
            &wasm_bindgen::JsValue::from_str("createObjectURL"),
        ) {
            Ok(v) if !v.is_null() && !v.is_undefined() => v,
            _ => return true,
        };
        let create_fn: &web_sys::js_sys::Function = match create_fn.dyn_ref() {
            Some(f) => f,
            None => return true,
        };
        match create_fn.call1(&url_obj, &file) {
            Ok(v) => match v.as_string() {
                Some(s) => s,
                None => return true,
            },
            Err(_) => return true,
        }
    };

    // Track index for updating after upload
    let idx = pending_images.read().len();
    pending_images.write().push(PendingImage {
        local_url,
        remote_url: None,
        uploading: true,
    });

    // Spawn upload
    spawn(async move {
        match upload_blob_to_s3(file).await {
            Ok(public_url) => {
                let mut imgs = pending_images.write();
                if let Some(img) = imgs.get_mut(idx) {
                    img.remote_url = Some(public_url);
                    img.uploading = false;
                }
            }
            Err(e) => {
                tracing::error!("Image upload failed: {e}");
                let mut imgs = pending_images.write();
                if idx < imgs.len() {
                    imgs.remove(idx);
                }
            }
        }
    });

    true
}

#[cfg(not(feature = "web"))]
pub fn handle_paste_event(
    _event: &ClipboardEvent,
    _pending_images: Signal<Vec<PendingImage>>,
) -> bool {
    false
}

#[cfg(feature = "web")]
async fn upload_blob_to_s3(file: web_sys::File) -> std::result::Result<String, String> {
    use wasm_bindgen::JsCast;

    // 1. Get presigned URL
    let presigned = crate::common::controllers::get_put_object_uri(
        Some(1),
        Some("comment-images".to_string()),
    )
    .await
    .map_err(|e| format!("Failed to get presigned URL: {e}"))?;

    let presigned_url = presigned
        .presigned_uris
        .first()
        .ok_or("No presigned URL returned")?
        .to_string();
    let public_url = presigned
        .uris
        .first()
        .ok_or("No public URL returned")?
        .to_string();

    // 2. PUT to S3
    let opts = web_sys::RequestInit::new();
    opts.set_method("PUT");
    let body = wasm_bindgen::JsValue::from(file.clone());
    opts.set_body(&body);

    let request = web_sys::Request::new_with_str_and_init(&presigned_url, &opts)
        .map_err(|e| format!("Failed to create request: {e:?}"))?;

    let content_type = file.type_();
    if !content_type.is_empty() {
        request
            .headers()
            .set("Content-Type", &content_type)
            .map_err(|e| format!("Failed to set content-type: {e:?}"))?;
    }

    let window = web_sys::window().ok_or("No window".to_string())?;
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Fetch failed: {e:?}"))?;
    let resp: web_sys::Response = resp_value
        .dyn_into()
        .map_err(|_| "Invalid response".to_string())?;

    if !resp.ok() {
        return Err(format!("Upload failed with status {}", resp.status()));
    }

    Ok(public_url)
}
