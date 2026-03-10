use crate::*;

pub async fn copy_text(text: &str) -> Result<()> {
    use wasm_bindgen_futures::JsFuture;

    let window = web_sys::window().ok_or(Error::OnlyWebFunction)?;
    let promise = window.navigator().clipboard().write_text(text);

    JsFuture::from(promise)
        .await
        .map_err(|_| Error::OnlyWebFunction)?;

    Ok(())
}
