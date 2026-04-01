use crate::common::*;

#[component]
pub fn LoadingIndicator(
    #[props(default = "300px".to_string())] max_width: String,
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let wrapper_class = if class.is_empty() {
        "flex col-span-full row-span-full justify-center items-center w-full min-h-[50vh] h-full grow max-tablet:max-w-[150px] max-tablet:mx-auto".to_string()
    } else {
        class
    };
    rsx! {
        div { class: "{wrapper_class}",
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
