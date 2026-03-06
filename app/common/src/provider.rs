use crate::*;

#[component]
pub fn Provider() -> Element {
    rsx! {
        Fragment {
            document::Script { src: "https://cdn.jsdelivr.net/npm/lucide@0.575.0/dist/cjs/lucide.min.js" }
            document::Script { src: asset!("/assets/ratel-common.js") }
            document::Script {
                r#type: "module",
                src: "https://unpkg.com/@lottiefiles/dotlottie-wc@latest/dist/dotlottie-wc.js",
            }

            document::Link {
                rel: "stylesheet",
                href: asset!("/assets/common-tailwind.css"),
            }
        }
    }
}
