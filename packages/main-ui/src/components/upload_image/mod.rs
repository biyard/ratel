#![allow(unused_variables)]
#![allow(unused_mut)]
use bdk::prelude::*;

#[cfg(feature = "web")]
use wasm_bindgen::JsCast;
#[component]
pub fn UploadImage(
    #[props(default = "file-upload".to_string())] id: String,
    onupload: EventHandler<FormEvent>,
    #[props(default = "image/*".to_string())] accept: String,
    #[props(default = false)] multiple: bool,
    children: Element,
) -> Element {
    rsx! {
        input {
            id: id.clone(),
            class: "hidden",
            r#type: "file",
            accept,
            multiple,
            onchange: {
                let id = id.clone();
                move |ev| {
                    onupload.call(ev);
                    #[cfg(feature = "web")]
                    {
                        let id = id.clone();
                        spawn(async move {
                            use gloo_timers::future::TimeoutFuture;
                            TimeoutFuture::new(0).await;
                            let input = web_sys::window()
                                .unwrap()
                                .document()
                                .unwrap()
                                .get_element_by_id(&id.clone())
                                .unwrap()
                                .dyn_into::<web_sys::HtmlInputElement>()
                                .unwrap();
                            input.set_value("");
                        });
                    }
                }
            },
        }
        {children}
    }
}
