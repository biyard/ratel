use crate::common::*;

#[component]
pub fn Provider() -> Element {
    rsx! {
        Fragment {
            document::Script { src: "https://cdn.jsdelivr.net/npm/lucide@0.575.0/dist/cjs/lucide.min.js" }
            document::Script {
                r#type: "module",
                src: "https://unpkg.com/@lottiefiles/dotlottie-wc@latest/dist/dotlottie-wc.js",
            }

        }
    }
}
