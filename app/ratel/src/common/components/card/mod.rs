use crate::common::*;

#[component]
pub fn Card(
    class: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let base_class = "rounded-[10px] bg-card-bg border border-card-border px-4 py-5";
    let class = match class {
        Some(extra) if !extra.is_empty() => format!("{} {}", base_class, extra),
        _ => base_class.to_string(),
    };

    rsx! {
        div { class, ..attributes, {children} }
    }
}
