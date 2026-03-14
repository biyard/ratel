use crate::common::*;

#[component]
pub fn Col(
    #[props(default)] main_axis_align: MainAxisAlign,
    #[props(default)] cross_axis_align: CrossAxisAlign,
    #[props(default)] class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col {main_axis_align} {cross_axis_align} {class}",
            ..attributes,
            {children}
        }
    }
}
