// use bdk::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

// Mapping Lottie JSON file structure to Rust structs
// https://github.com/airbnb/lottie-web/blob/master/docs/json/animation.json
#[derive(Debug, Serialize, Deserialize)]
pub struct LottieAnimationData {
    pub v: String,          // version
    pub fr: f32,            // frame rate
    pub ip: f32,            // inpoint (start frame)
    pub op: f32,            // outpoint (end frame)
    pub w: f32,             // width
    pub h: f32,             // height
    pub nm: Option<String>, // name
    // pub assets: Option<Vec<Asset>>,
    pub layers: Vec<Layer>,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Asset {
//     pub id: String,
//     #[serde(rename = "p")]
//     pub path: Option<String>,
//     #[serde(rename = "u")]
//     pub path_prefix: Option<String>,
//     #[serde(rename = "e")]
//     pub embedded: Option<u8>,
//     #[serde(rename = "layers")]
//     pub layers: Option<Vec<Layer>>,
// }

// https://github.com/airbnb/lottie-web/blob/master/docs/json/layers/text.json
#[derive(Debug, Serialize, Deserialize)]
pub struct Layer {
    pub ty: u8, // layer type (0: PreComp, 1: Solid, 2: Image, 3: NULL, 4: Shape, 5: Text)
    pub ks: Option<Transform>, // transform
    pub ind: Option<u32>, // index
    pub ip: Option<f32>, // inpoint (start frame)
    pub op: Option<f32>, // outpoint (end frame)
    pub st: Option<f32>, // start frame
    pub nm: Option<String>, // name
    pub parent: Option<u32>, // parent layer index
    pub sr: Option<f32>, // layer time stretching
    pub shapes: Option<Vec<Shape>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transform {
    pub a: Option<AnimatableValue>, // Anchor Point
    pub p: Option<AnimatableValue>, // Position
    pub s: Option<AnimatableValue>, // Scale
    pub r: Option<AnimatableValue>, // Rotation
    pub o: Option<AnimatableValue>, // Opacity
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimatableValue {
    pub k: serde_json::Value, // Keyframes or static value
    pub x: Option<String>,    // Expression
    pub ix: Option<u32>,      // Property index
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shape {
    pub ty: String,                  // type (gr: group, sh: pass, fl: fill, st: stroke)
    pub nm: Option<String>,          // name
    pub it: Option<Vec<Shape>>,      // items (sub-shapes)
    pub d: Option<u8>,               // direction (0: ccw, 1: cw)
    pub pt: Option<Vec<ShapePoint>>, // points (for path shapes)
    pub c: Option<AnimatableValue>,  // color (for fill shapes)
    pub o: Option<AnimatableValue>,  // opacity (for fill shapes)
    pub w: Option<AnimatableValue>,  // width (for stroke shapes)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShapePoint {
    pub x: f32,
    pub y: f32,
}

pub struct LottieRenderer {
    animation: LottieAnimationData,
    current_frame: f32,
}

impl LottieRenderer {
    pub fn new(json_data: &str) -> Result<Self, serde_json::Error> {
        let animation: LottieAnimationData = serde_json::from_str(json_data)?;
        Ok(Self {
            animation,
            current_frame: 0.0,
        })
    }

    pub fn get_duration(&self) -> f32 {
        (self.animation.op - self.animation.ip) / self.animation.fr
    }

    pub fn get_dimensions(&self) -> (f32, f32) {
        (self.animation.w, self.animation.h)
    }

    pub fn advance_frame(&mut self, delta_time: f32) {
        let frame_advance = delta_time * self.animation.fr;
        self.current_frame += frame_advance;

        // loop animation
        if self.current_frame > self.animation.op {
            self.current_frame = self.animation.ip;
        }
    }

    // Rendering to SVG format
    pub fn render_to_svg(&self) -> String {
        let (width, height) = self.get_dimensions();

        let mut svg = format!(
            r#"<svg width="{}" height="{}" viewBox="0 0 {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height, width, height
        );

        // Render layers in reverse order (Lottie draws the last layer at the bottom)
        for layer in self.animation.layers.iter().rev() {
            if let Some(rendered_layer) = self.render_layer(layer) {
                svg.push_str(&rendered_layer);
            }
        }

        svg.push_str("</svg>");
        svg
    }

    fn render_layer(&self, layer: &Layer) -> Option<String> {
        // current_frame start < layer.ip && current_frame < layer.op
        let in_point = layer.ip.unwrap_or(self.animation.ip);
        let out_point = layer.op.unwrap_or(self.animation.op);

        if self.current_frame < in_point || self.current_frame >= out_point {
            return None;
        }

        // 0: PreComp, 1: Solid, 2: Image, 3: NULL, 4: Shape, 5: Text
        match layer.ty {
            4 => self.render_shape_layer(layer), // only Shape layer
            _ => None,
        }
    }

    fn render_shape_layer(&self, layer: &Layer) -> Option<String> {
        if let Some(shapes) = &layer.shapes {
            let mut result = String::new();

            // 레이어의 트랜스폼 적용 (필요한 경우)
            let transform_attr = if let Some(transform) = &layer.ks {
                self.get_transform_attributes(transform)
            } else {
                String::new()
            };

            result.push_str(&format!("<g {}>\n", transform_attr));

            // render all shape
            for shape in shapes {
                if let Some(rendered_shape) = self.render_shape(shape) {
                    result.push_str(&rendered_shape);
                }
            }

            result.push_str("</g>\n");
            Some(result)
        } else {
            None
        }
    }

    fn render_shape(&self, shape: &Shape) -> Option<String> {
        match shape.ty.as_str() {
            "gr" => self.render_group_shape(shape),
            "sh" => self.render_path_shape(shape),
            "fl" => self.render_fill_shape(shape),
            "st" => self.render_stroke_shape(shape),
            _ => None,
        }
    }

    fn render_group_shape(&self, shape: &Shape) -> Option<String> {
        if let Some(items) = &shape.it {
            let mut result = String::from("<g>\n");

            for item in items {
                if let Some(rendered_item) = self.render_shape(item) {
                    result.push_str(&rendered_item);
                }
            }

            result.push_str("</g>\n");
            Some(result)
        } else {
            None
        }
    }

    fn render_path_shape(&self, shape: &Shape) -> Option<String> {
        // need bezier pass
        Some(String::from(
            "<path d=\"M0,0 L100,100\" stroke=\"black\" fill=\"none\" />\n",
        ))
    }

    fn render_fill_shape(&self, shape: &Shape) -> Option<String> {
        Some(String::from(
            "<rect width=\"100\" height=\"100\" fill=\"blue\" />\n",
        ))
    }

    fn render_stroke_shape(&self, shape: &Shape) -> Option<String> {
        Some(String::from(
            "<rect width=\"100\" height=\"100\" stroke=\"red\" fill=\"none\" />\n",
        ))
    }

    fn get_transform_attributes(&self, transform: &Transform) -> String {
        "transform=\"translate(0,0) scale(1,1) rotate(0)\"".to_string()
    }
}

#[wasm_bindgen]
pub struct LottiePlayer {
    renderer: LottieRenderer,
    last_time: f64,
}

#[wasm_bindgen]
impl LottiePlayer {
    #[wasm_bindgen(constructor)]
    pub fn new(json_data: &str) -> Result<LottiePlayer, JsValue> {
        match LottieRenderer::new(json_data) {
            Ok(renderer) => Ok(LottiePlayer {
                renderer,
                last_time: js_sys::Date::now(),
            }),
            Err(e) => Err(JsValue::from_str(&format!(
                "Error parsing Lottie JSON: {:?}",
                e
            ))),
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) -> String {
        let current_time = js_sys::Date::now();
        let delta_time = (current_time - self.last_time) / 1000.0;
        self.last_time = current_time;

        self.renderer.advance_frame(delta_time as f32);
        self.renderer.render_to_svg()
    }

    #[wasm_bindgen]
    pub fn get_duration(&self) -> f32 {
        self.renderer.get_duration()
    }

    #[wasm_bindgen]
    pub fn get_dimensions(&self) -> Box<[f32]> {
        let (width, height) = self.renderer.get_dimensions();
        Box::new([width, height])
    }
}

// #[component]
// pub fn LottieAnimation(
//     json_data: String,
//     #[props(default = "".to_string())] class: String,
//     #[props(default = "100%".to_string())] width: String,
//     #[props(default = "100%".to_string())] height: String,
// ) -> Element {
//     let player = use_signal(|| None);
//     let svg_content = use_signal(|| String::new());
//     let animation_frame = use_signal(|| None::<i32>);

//     use_effect(move || {
//         let result = LottiePlayer::new(&json_data);
//         if let Ok(lottie_player) = result {
//             player.set(Some(lottie_player));
//         }
//     });

//     // 애니메이션 프레임 업데이트 함수
//     let update_animation = move || {
//         if let Some(player) = player.get() {
//             let mut player_clone = player.clone();
//             let new_svg = player_clone.update();
//             svg_content.set(new_svg);

//             // 다음 프레임 요청
//             let window = web_sys::window().expect("no global window exists");
//             let callback = Closure::once(Box::new(move || {
//                 update_animation();
//             }) as Box<dyn FnOnce()>);

//             let id = window
//                 .request_animation_frame(callback.as_ref().unchecked_ref())
//                 .expect("failed to request animation frame");
//             animation_frame.set(Some(id));
//             callback.forget(); // 메모리 누수 방지를 위해 적절한 정리 로직 필요
//         }
//     };

//     // 컴포넌트 마운트 시 애니메이션 시작
//     use_effect(move || {
//         update_animation();

//         // 컴포넌트 언마운트 시 애니메이션 정리
//         move || {
//             if let Some(id) = animation_frame.get() {
//                 let window = web_sys::window().expect("no global window exists");
//                 window.cancel_animation_frame(id);
//             }
//             ()
//         }
//     });

//     rsx! {
//         div {
//             class: "{class}",
//             style: "width: {width}; height: {height};",
//             dangerous_inner_html: "{svg_content}",
//         }
//     }
// }

#[allow(dead_code)]
fn load_lottie_from_file(file_path: &str) -> String {
    let file = std::fs::File::open(file_path).expect("Unable to open file");
    let reader = std::io::BufReader::new(file);
    let json_data: LottieAnimationData =
        serde_json::from_reader(reader).expect("Unable to parse JSON");
    serde_json::to_string(&json_data).expect("Unable to convert to string")
}
