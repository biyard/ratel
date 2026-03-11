use crate::common::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumString,
    Default,
)]
pub enum InputType {
    #[default]
    #[strum(serialize = "text")]
    Text,
    #[strum(serialize = "email")]
    Email,
    #[strum(serialize = "number")]
    Number,
    #[strum(serialize = "password")]
    Password,
}

#[component]
pub fn Input(
    #[props(default)] variant: InputVariant,
    #[props(default)] class: String,
    #[props(default)] r#type: InputType,
    #[props(default)] value: String,
    #[props(optional)] placeholder: Option<String>,
    #[props(default)] maxlength: usize,
    #[props(default = "off".to_string())] autocomplete: String,
    #[props(default)] disabled: bool,
    #[props(default)] name: String,
    #[props(default)] oninput: EventHandler<FormEvent>,
    #[props(default)] onchange: EventHandler<FormEvent>,
    #[props(default)] onkeydown: EventHandler<KeyboardEvent>,
    #[props(default)] onblur: EventHandler<FocusEvent>,
    #[props(default)] onconfirm: EventHandler<KeyboardEvent>, // Keydown event with Enter key
    #[props(default)] oncancel: EventHandler<KeyboardEvent>,  // Keydown event with Escape key
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
) -> Element {
    let mut attributes = attributes;
    if maxlength > 0 {
        attributes.push(Attribute::new(
            "maxlength",
            maxlength.to_string(),
            None,
            false,
        ));
    }

    rsx! {
        input {
            r#type: r#type.to_string(),
            class: "{variant} {class}",
            name,
            value,
            placeholder,
            disabled,
            autocomplete,
            oninput,
            onkeydown: move |evt: KeyboardEvent| {
                if evt.key() == Key::Enter {
                    debug!("Enter key pressed, triggering onconfirm");
                    onconfirm.call(evt.clone());
                } else if evt.key() == Key::Escape {
                    debug!("Escape key pressed, triggering oncancel");
                    oncancel.call(evt.clone());
                }
                if evt.propagates() {
                    onkeydown.call(evt);
                }
            },
            onblur,
            onchange,
            ..attributes,
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    DeserializeFromStr,
    SerializeDisplay,
    strum::Display,
    strum::EnumString,
    Default,
)]
pub enum InputVariant {
    #[default]
    #[strum(
        serialize = "flex px-5 w-full min-w-0 h-9 text-base font-light border outline-none md:text-sm disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none shadow-xs transition-[color,box-shadow] file:text-text-primary file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground aria-invalid:ring-destructive/20 aria-invalid:outline aria-invalid:border-c-p-50 bg-input-box-bg border-input-box-border rounded-[10px] py-5.5 text-text-primary dark:bg-input/30 dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[1px]"
    )]
    Default,
    #[strum(serialize = "")]
    Plain,
}
