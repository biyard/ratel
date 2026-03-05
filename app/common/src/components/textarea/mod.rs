use crate::*;

#[component]
pub fn TextArea(
    #[props(default)] value: String,
    #[props(default)] placeholder: String,
    #[props(default)] class: String,
    #[props(default)] disabled: bool,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    rsx! {
        textarea {
            class: "{class}",
            value: "{value}",
            placeholder: "{placeholder}",
            disabled,
            oninput: move |e| {
                if let Some(handler) = &oninput {
                    handler.call(e);
                }
            },
            ..attributes,
        }
    }
}
