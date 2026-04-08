use crate::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Serialize,
    Deserialize,
    Default,
)]
pub enum Platform {
    #[default]
    Desktop,
    Mobile,
    Tablet,
}

pub fn use_platform() -> Signal<Platform> {
    let mut platform = use_signal(|| {
        #[cfg(not(feature = "server"))]
        {
            let window = web_sys::window().unwrap();
            let width = window.inner_width().unwrap().as_f64().unwrap_or_default();

            if width < 550.0 {
                Platform::Mobile
            } else if width < 1024.0 {
                Platform::Tablet
            } else {
                Platform::Desktop
            }
        }

        #[cfg(feature = "server")]
        {
            use dioxus::fullstack::FullstackContext;
            let Some(ctx) = FullstackContext::current() else {
                return Platform::Desktop;
            };

            let parts = ctx.parts_mut();
            let user_agent = parts
                .headers
                .get("user-agent")
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default();


            if user_agent.contains("mobile") {
                Platform::Mobile
            } else if user_agent.contains("tablet") {
                Platform::Tablet
            } else {
                Platform::Desktop
            }
        }
    });

    #[cfg(not(feature = "server"))]
    {
        use_effect(move || {
            use wasm_bindgen::closure::Closure;
            use wasm_bindgen::JsCast;

            let closure = Closure::wrap(Box::new(move || {
                let width = web_sys::window().unwrap().inner_width().unwrap().as_f64().unwrap_or_default();
                platform.set(if width < 550.0 {
                    Platform::Mobile
                } else if width < 1024.0 {
                    Platform::Tablet
                } else {
                    Platform::Desktop
                });
            }) as Box<dyn FnMut()>);
            web_sys::window().unwrap()
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        });
    }

    platform
}
