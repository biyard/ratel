use crate::*;

#[component]
pub fn LoadingIndicator(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div { class: "flex col-span-full row-span-full justify-center items-center w-full h-full grow",
            dotlottie-wc {
                src: asset!("/assets/animations/loading.lottie"),
                "autoplay": true,
                "loop": true,
                ..attributes,
            }
        }
    }
}
