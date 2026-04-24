use super::*;

#[component]
pub fn StartTimeSetting(space_id: ReadSignal<SpacePartition>) -> Element {
    let space = use_space();
    let tr: GeneralTranslate = use_translate();
    let UseSpaceGeneralSettings {
        mut update_start_time,
        ..
    } = use_space_general_settings(space_id)?;

    let started_at = space().started_at;
    let input_value = started_at
        .map(crate::common::utils::time::epoch_ms_to_datetime_local)
        .unwrap_or_else(|| {
            crate::common::utils::time::epoch_ms_to_datetime_local(space().created_at)
        });

    rsx! {
        section { class: "sga-section", "data-testid": "section-start-time",
            div { class: "sga-section__head",
                span { class: "sga-section__label", "{tr.start_time_setting}" }
                span { class: "sga-section__hint", "{tr.start_time_description}" }
            }
            div { class: "sga-field",
                label { class: "sga-field__label", "Started at" }
                input {
                    class: "sga-input sga-input--datetime",
                    r#type: "datetime-local",
                    value: "{input_value}",
                    "data-testid": "start-time-input",
                    // Chromium/Firefox: clicking anywhere on the field
                    // only opens the picker if we explicitly call
                    // `showPicker()`. Without this, only the built-in
                    // calendar icon is a hot-zone.
                    onclick: move |e: MouseEvent| {
                        #[cfg(feature = "web")]
                        {
                            use dioxus::web::WebEventExt;
                            use wasm_bindgen::JsCast;
                            if let Some(web_event) = e.try_as_web_event() {
                                if let Some(input) = web_event
                                    .target()
                                    .and_then(|t| { t.dyn_into::<web_sys::HtmlInputElement>().ok() })
                                {
                                    let _ = input.show_picker();
                                }
                            }
                        }
                        #[cfg(not(feature = "web"))]
                        let _ = e;
                    },
                    onchange: move |e: FormEvent| {
                        let raw = e.value();
                        if let Some(ms) =
                            crate::common::utils::time::datetime_local_to_epoch_ms(&raw)
                        {
                            update_start_time.call(ms);
                        }
                    },
                }
            }
        }
    }
}
