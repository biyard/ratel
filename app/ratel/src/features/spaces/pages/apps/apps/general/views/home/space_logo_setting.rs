use super::*;

fn initials_for(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

#[component]
pub fn SpaceLogoSetting(space_id: ReadSignal<SpacePartition>) -> Element {
    let space = use_space();
    let tr: GeneralTranslate = use_translate();
    let UseSpaceGeneralSettings {
        mut update_logo, ..
    } = use_space_general_settings(space_id)?;

    let logo = space().logo.clone();
    let placeholder_initials = initials_for(&space().title);
    let pending = update_logo.pending();

    rsx! {
        section { class: "sga-section", "data-testid": "section-space-logo",
            div { class: "sga-section__head",
                span { class: "sga-section__label", "{tr.space_logo}" }
                span { class: "sga-section__hint", "{tr.space_logo_description}" }
            }
            div { class: "sga-logo-uploader",
                // Avatar is visual-only — upload must happen via the
                // "Upload logo" pill so the click surface matches the
                // user's mental model (label says Upload, the click
                // should originate there).
                if logo.is_empty() {
                    div {
                        class: "sga-logo-preview sga-logo-preview--static",
                        "data-testid": "space-logo-preview",
                        "{placeholder_initials}"
                    }
                } else {
                    div {
                        class: "sga-logo-preview sga-logo-preview--static",
                        "data-testid": "space-logo-preview",
                        img { src: "{logo}", alt: "Space logo" }
                    }
                }
                div { class: "sga-logo-uploader__actions",
                    // Upload Logo — style the FileUploader's own <label>
                    // as the accent button. A <button> nested inside a
                    // <label> does NOT trigger the associated file input
                    // (HTML spec: form-control children of a label don't
                    // delegate), so we promote the label itself.
                    FileUploader {
                        class: "sga-btn sga-btn--accent",
                        on_upload_success: move |url: String| update_logo.call(url),
                        {tr.upload_logo}
                    }
                    if !logo.is_empty() {
                        button {
                            r#type: "button",
                            class: "sga-btn",
                            "data-testid": "space-logo-remove",
                            disabled: pending,
                            onclick: move |_| update_logo.call(String::new()),
                            "Remove"
                        }
                    }
                }
            }
        }
    }
}
