use crate::common::*;

#[component]
pub fn LoadingIndicator(
    #[props(default = "300px".to_string())] max_width: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div { class: "flex col-span-full row-span-full justify-center items-center w-full min-h-[50vh] h-full grow max-tablet:max-w-[150px] max-tablet:mx-auto",
            dotlottie-wc {
                src: asset!("/assets/animations/loading.lottie"),
                "autoplay": true,
                "loop": true,
                max_width,
                ..attributes,
            }
        }
    }
}
