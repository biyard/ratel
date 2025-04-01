use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/lottie.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = "loadAnimation")]
    pub async fn load_animation(
        container_id: &str,
        animation_path: &str,
        options: JsValue,
    ) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = "stopAnimation")]
    pub fn stop_animation(container_id: &str);

    #[wasm_bindgen(js_name = "playAnimation")]
    pub fn play_animation(container_id: &str);

    #[wasm_bindgen(js_name = "pauseAnimation")]
    pub fn pause_animation(container_id: &str);

    #[wasm_bindgen(js_name = "goToFrame")]
    pub fn go_to_frame(container_id: &str, frame: f64);

    #[wasm_bindgen(js_name = "setSpeed")]
    pub fn set_speed(container_id: &str, speed: f64);

    #[wasm_bindgen(js_name = "destroyAnimation")]
    pub fn destroy_animation(container_id: &str);

    #[wasm_bindgen(js_name = "destroyAllAnimations")]
    pub fn destroy_all_animations();
}
#[derive(Clone, Debug)]
pub struct LottieController {
    container_id: String,
}

impl LottieController {
    pub async fn load(
        container_id: &str,
        animation_path: &str,
        loop_animation: bool,
        autoplay: bool,
    ) -> Result<Self, String> {
        let options = js_sys::Object::new().into();
        let _ = js_sys::Reflect::set(&options, &"loop".into(), &loop_animation.into());
        let _ = js_sys::Reflect::set(&options, &"autoplay".into(), &autoplay.into());

        match load_animation(container_id, animation_path, options).await {
            Ok(v) => {
                tracing::debug!("Animation loaded {:?}", v);
                Ok(Self {
                    container_id: container_id.to_string(),
                })
            }
            Err(e) => {
                tracing::error!("Failed to load animation: {:?}", e);
                Err(format!("Failed to load animation: {:?}", e))
            }
        }
    }

    pub fn stop(&self) {
        stop_animation(&self.container_id);
    }

    pub fn play(&self) {
        play_animation(&self.container_id);
    }

    pub fn pause(&self) {
        pause_animation(&self.container_id);
    }

    pub fn go_to_frame(&self, frame: f64) {
        go_to_frame(&self.container_id, frame);
    }

    pub fn set_speed(&self, speed: f64) {
        set_speed(&self.container_id, speed);
    }
}

impl Drop for LottieController {
    fn drop(&mut self) {
        destroy_animation(&self.container_id);
    }
}

pub fn cleanup_all_animations() {
    destroy_all_animations();
}
