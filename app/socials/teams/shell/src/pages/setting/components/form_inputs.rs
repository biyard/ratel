use super::super::*;

#[component]
pub fn Input(
    value: String,
    #[props(optional)] placeholder: Option<String>,
    #[props(optional)] disabled: Option<bool>,
    #[props(optional)] class: Option<String>,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let class_name = class.unwrap_or_default();
    let placeholder = placeholder.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);

    rsx! {
        input {
            class: "{class_name}",
            r#type: "text",
            value: "{value}",
            placeholder: "{placeholder}",
            disabled,
            oninput: move |e| {
                if let Some(handler) = &oninput {
                    handler.call(e);
                }
            },
        }
    }
}

#[component]
pub fn TextArea(
    value: String,
    #[props(optional)] placeholder: Option<String>,
    #[props(optional)] disabled: Option<bool>,
    #[props(optional)] class: Option<String>,
    #[props(optional)] oninput: Option<EventHandler<FormEvent>>,
) -> Element {
    let class_name = class.unwrap_or_default();
    let placeholder = placeholder.unwrap_or_default();
    let disabled = disabled.unwrap_or(false);

    rsx! {
        textarea {
            class: "{class_name}",
            value: "{value}",
            placeholder: "{placeholder}",
            disabled,
            oninput: move |e| {
                if let Some(handler) = &oninput {
                    handler.call(e);
                }
            },
        }
    }
}
