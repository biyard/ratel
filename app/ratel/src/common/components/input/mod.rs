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
    #[props(default = "off".to_string())] autocomplete: String,
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    oninvalid: Option<EventHandler<FormEvent>>,
    onselect: Option<EventHandler<SelectionEvent>>,
    onselectionchange: Option<EventHandler<SelectionEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onfocusin: Option<EventHandler<FocusEvent>>,
    onfocusout: Option<EventHandler<FocusEvent>>,
    onkeypress: Option<EventHandler<KeyboardEvent>>,
    onkeyup: Option<EventHandler<KeyboardEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    oncompositionstart: Option<EventHandler<CompositionEvent>>,
    oncompositionupdate: Option<EventHandler<CompositionEvent>>,
    oncompositionend: Option<EventHandler<CompositionEvent>>,
    oncopy: Option<EventHandler<ClipboardEvent>>,
    oncut: Option<EventHandler<ClipboardEvent>>,
    onpaste: Option<EventHandler<ClipboardEvent>>,
    onconfirm: Option<EventHandler<KeyboardEvent>>, // Enter key inside the input
    oncancel: Option<EventHandler<KeyboardEvent>>,  // Escape key inside the input
    #[props(extends=GlobalAttributes)]
    #[props(extends=input)]
    attributes: Vec<Attribute>,
) -> Element {
    let merged = dioxus_primitives::merge_attributes(vec![
        dioxus_primitives::dioxus_attributes::attributes!(input {
            r#type: r#type.to_string(),
            class: "{variant} {class}",
            name: if &autocomplete != "off" {
                autocomplete.clone()
            },
            id: if &autocomplete != "off" {
                autocomplete.clone()
            },
            autocomplete,
        }),
        attributes,
    ]);

    rsx! {
        input {
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
                handle_confirm_cancel(&evt, onconfirm, oncancel, onkeydown);
            },
            ..merged,
        }
    }
}

/// Dispatches [`onconfirm`] on Enter and [`oncancel`] on Escape while
/// tolerating browser autofill events.
///
/// Browser autofill can dispatch synthetic keydown events where `key` and
/// `isComposing` are `undefined`, which panics the default wasm-bindgen
/// conversions used by `KeyboardEvent::key()` / `KeyboardEvent::is_composing()`.
/// We work around that by reading the raw JS properties defensively — see
/// <https://github.com/DioxusLabs/dioxus/pull/4492>.
#[cfg(feature = "web")]
fn handle_confirm_cancel(
    evt: &KeyboardEvent,
    onconfirm: Option<EventHandler<KeyboardEvent>>,
    oncancel: Option<EventHandler<KeyboardEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
) {
    use dioxus::web::WebEventExt;
    use wasm_bindgen::{JsCast, JsValue};
    use web_sys::js_sys::Reflect;

    let Some(web_event) = evt.try_as_web_event() else {
        return;
    };
    let js_obj: &JsValue = web_event.unchecked_ref();

    let is_composing = Reflect::get(js_obj, &JsValue::from_str("isComposing"))
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if is_composing {
        return;
    }

    let key = Reflect::get(js_obj, &JsValue::from_str("key"))
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_default();

    if key == "Enter" {
        evt.prevent_default();
        debug!("Enter key pressed, triggering onconfirm");
        if let Some(onconfirm) = onconfirm {
            onconfirm.call(evt.clone());
        }
    } else if key == "Escape" {
        evt.prevent_default();
        debug!("Escape key pressed, triggering oncancel");
        if let Some(oncancel) = oncancel {
            oncancel.call(evt.clone());
        }
    }

    if evt.propagates() {
        if let Some(onkeydown) = onkeydown {
            onkeydown.call(evt.clone());
        }
    }
}

#[cfg(not(feature = "web"))]
fn handle_confirm_cancel(
    _evt: &KeyboardEvent,
    _onconfirm: Option<EventHandler<KeyboardEvent>>,
    _oncancel: Option<EventHandler<KeyboardEvent>>,
    _onkeydown: Option<EventHandler<KeyboardEvent>>,
) {
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
        serialize = "flex px-5 w-full min-w-0 h-9 text-base font-light border outline-none md:text-sm disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none shadow-xs transition-[color,box-shadow] file:text-text-primary file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium selection:bg-primary selection:text-primary-foreground placeholder:text-muted-foreground aria-invalid:ring-destructive/20 aria-invalid:outline aria-invalid:border-c-p-50 bg-input-box-bg border-input-box-border rounded-[10px] py-5.5 text-text-primary dark:bg-input/30 dark:aria-invalid:ring-destructive/40 focus-visible:border-[color:rgba(252,179,0,0.6)] focus-visible:outline-none focus-visible:shadow-[0_0_0_3px_rgba(252,179,0,0.2),var(--rim-glow-primary)]"
    )]
    Default,
    #[strum(serialize = "")]
    Plain,
}
