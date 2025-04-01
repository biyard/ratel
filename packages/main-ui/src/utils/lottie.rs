use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/deps/js/lottie.js")]
extern "C" {
    #[wasm_bindgen(catch, js_name = "loadAnimation")]
    fn load_animation(container_id: &str, animation_path: &str, options: JsValue) -> Promise;

    #[wasm_bindgen(catch, js_name = "stopAnimation")]
    fn stop_animation(container_id: &str);

    #[wasm_bindgen(catch, js_name = "playAnimation")]
    fn play_animation(container_id: &str);

    #[wasm_bindgen(catch, js_name = "pauseAnimation")]
    fn pause_animation(container_id: &str);

    #[wasm_bindgen(catch, js_name = "goToFrame")]
    fn go_to_frame(container_id: &str, frame: f64);

    #[wasm_bindgen(catch, js_name = "setSpeed")]
    fn set_speed(container_id: &str, speed: f64);

    #[wasm_bindgen(catch, js_name = "destroyAnimation")]
    fn destroy_animation(container_id: &str);

    #[wasm_bindgen(catch, js_name = "destroyAllAnimations")]
    fn destroy_all_animations();
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
            Ok(_) => {
                tracing::debug!("Animation loaded successfully");
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
