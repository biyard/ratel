use crate::common::*;

#[component]
pub fn Provider() -> Element {
    rsx! {
        Fragment {
            // The lucide CDN script was removed — it served the CJS build
            // (`/dist/cjs/lucide.min.js`) which throws
            // `Identifier 'Infinity' has already been declared` when loaded
            // as a browser <script>. Nothing in the codebase actually
            // references the global `lucide` namespace; icons are rendered
            // through the `lucide-dioxus` Rust crate at compile time.
            document::Script {
                r#type: "module",
                src: "https://unpkg.com/@lottiefiles/dotlottie-wc@latest/dist/dotlottie-wc.js",
            }
        }
    }
}
