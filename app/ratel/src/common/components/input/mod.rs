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
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    oninvalid: Option<EventHandler<FormEvent>>,
    onselect: Option<EventHandler<SelectionEvent>>,
    onselectionchange: Option<EventHandler<SelectionEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onfocusin: Option<EventHandler<FocusEvent>>,
    onfocusout: Option<EventHandler<FocusEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    onkeypress: Option<EventHandler<KeyboardEvent>>,
    onkeyup: Option<EventHandler<KeyboardEvent>>,
    oncompositionstart: Option<EventHandler<CompositionEvent>>,
    oncompositionupdate: Option<EventHandler<CompositionEvent>>,
    oncompositionend: Option<EventHandler<CompositionEvent>>,
    oncopy: Option<EventHandler<ClipboardEvent>>,
    oncut: Option<EventHandler<ClipboardEvent>>,
    onpaste: Option<EventHandler<ClipboardEvent>>,
    onconfirm: Option<EventHandler<KeyboardEvent>>, // Keydown event with Enter key
    oncancel: Option<EventHandler<KeyboardEvent>>,  // Keydown event with Escape key
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
            oninput: move |e| _ = oninput.map(|callback| callback(e)),
            onchange: move |e| _ = onchange.map(|callback| callback(e)),
            oninvalid: move |e| _ = oninvalid.map(|callback| callback(e)),
            onselect: move |e| _ = onselect.map(|callback| callback(e)),
            onselectionchange: move |e| _ = onselectionchange.map(|callback| callback(e)),
            onfocus: move |e| _ = onfocus.map(|callback| callback(e)),
            onblur: move |e| _ = onblur.map(|callback| callback(e)),
            onfocusin: move |e| _ = onfocusin.map(|callback| callback(e)),
            onfocusout: move |e| _ = onfocusout.map(|callback| callback(e)),
            onkeypress: move |e| _ = onkeypress.map(|callback| callback(e)),
            onkeyup: move |e| _ = onkeyup.map(|callback| callback(e)),
            oncompositionstart: move |e| _ = oncompositionstart.map(|callback| callback(e)),
            oncompositionupdate: move |e| _ = oncompositionupdate.map(|callback| callback(e)),
            oncompositionend: move |e| _ = oncompositionend.map(|callback| callback(e)),
            oncopy: move |e| _ = oncopy.map(|callback| callback(e)),
            oncut: move |e| _ = oncut.map(|callback| callback(e)),
            onpaste: move |e| _ = onpaste.map(|callback| callback(e)),
            onkeydown: move |evt: KeyboardEvent| {
                if evt.key() == Key::Enter {
                    debug!("Enter key pressed, triggering onconfirm");
                    if let Some(onconfirm) = &onconfirm {
                        onconfirm.call(evt.clone());
                    }
                } else if evt.key() == Key::Escape {
                    debug!("Escape key pressed, triggering oncancel");
                    if let Some(oncancel) = &oncancel {
                        oncancel.call(evt.clone());
                    }
                }
                if evt.propagates() {
                    if let Some(onkeydown) = &onkeydown {
                        onkeydown.call(evt);
                    }
                }
            },
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
