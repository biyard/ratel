use crate::*;

#[component]
pub fn LoadingIndicator(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div {..attributes,
            dotlottie-wc {
                src: asset!("/assets/animations/loading.lottie"),
                "autoplay": true,
                "loop": true,
            }
        }
    }
}
