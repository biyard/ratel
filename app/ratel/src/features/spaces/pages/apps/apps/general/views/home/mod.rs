use super::*;

mod administrators;
mod anonymous_setting;
mod delete_space_popup;
mod invite_participant;
mod join_anytime_setting;
mod space_logo_setting;
mod space_visibility_setting;
mod start_time_setting;

use administrators::*;
use anonymous_setting::*;
use delete_space_popup::*;
use invite_participant::*;
use join_anytime_setting::*;
use space_logo_setting::*;
use space_visibility_setting::*;
use start_time_setting::*;

const DEFAULT_PROFILE_IMAGE: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";
// Square symbol mark — safer for the 36x36 topbar tile than the wordmark
// (`logo.png`), which is horizontally laid out and gets clipped.
const DEFAULT_SPACE_LOGO: &str = "https://metadata.ratel.foundation/logos/logo-symbol.png";

fn normalize_email_inputs(raw: &str) -> Result<Vec<String>> {
    let emails: Vec<String> = raw
        .split(',')
        .map(|value| value.trim().to_ascii_lowercase())
        .filter(|value| !value.is_empty())
        .collect();

    if emails.is_empty() {
        return Err(Error::InvalidEmail);
    }

    let mut normalized = Vec::new();
    for email in emails {
        if !email.contains('@') {
            return Err(Error::InvalidEmail);
        }

        if !normalized.iter().any(|value| value == &email) {
            normalized.push(email);
        }
    }

    Ok(normalized)
}

fn normalize_identifier_inputs(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect()
}

#[component]
pub fn SpaceGeneralAppPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: GeneralTranslate = use_translate();
    let space = use_space();
    let nav = use_navigator();
    let mut popup = use_popup();

    // Instantiate the controller hook once at the page root. Child
    // components pick it up via the same `use_space_general_settings(..)?`
    // call; the `try_use_context` early return shares the same instance.
    let UseSpaceGeneralSettings {
        mut delete_space_action,
        ..
    } = use_space_general_settings(space_id)?;

    let space_data = space();
    let space_logo = if space_data.logo.is_empty() {
        DEFAULT_SPACE_LOGO.to_string()
    } else {
        space_data.logo.clone()
    };
    let space_title = space_data.title.clone();

    rsx! {
        document::Link { rel: "preload", href: asset!("./style.css"), r#as: "style" }
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div { class: "space-general-arena",

            // ── Arena topbar ────────────────────────────
            header { class: "sga-topbar", role: "banner",
                div { class: "sga-topbar__left",
                    button {
                        r#type: "button",
                        class: "sga-back-btn",
                        "aria-label": "Back",
                        "data-testid": "topbar-back",
                        onclick: move |_| {
                            // Follow browser history — whichever page
                            // the user was on before landing here.
                            nav.go_back();
                        },
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            path { d: "M19 12H5" }
                            path { d: "M12 19l-7-7 7-7" }
                        }
                    }
                    img {
                        class: "sga-topbar__logo",
                        alt: "Space logo",
                        src: "{space_logo}",
                    }
                    nav { class: "sga-breadcrumb",
                        span { class: "sga-breadcrumb__item", "{space_title}" }
                        span { class: "sga-breadcrumb__sep", "›" }
                        span { class: "sga-breadcrumb__item", "Apps" }
                        span { class: "sga-breadcrumb__sep", "›" }
                        span { class: "sga-breadcrumb__item sga-breadcrumb__current",
                            "General"
                        }
                    }
                    span { class: "sga-type-badge", "data-testid": "type-badge",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            circle { cx: "12", cy: "12", r: "3" }
                            path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" }
                        }
                        "General"
                    }
                    span { class: "sga-topbar-title", "General Settings" }
                }
            }

            // ── Main body ───────────────────────────────
            main { class: "sga-body",
                h1 { class: "sga-body__title", "{tr.space_setting}" }

                SpaceLogoSetting { space_id }
                StartTimeSetting { space_id }
                SpaceVisibilitySetting { space_id }
                InviteParticipant { space_id }
                AnonymousSetting { space_id }
                JoinAnytimeSetting { space_id }
                Administrators { space_id }

                // Danger zone
                section { class: "sga-section sga-section--danger",
                    div { class: "sga-section__head",
                        span {
                            class: "sga-section__label",
                            style: "color:var(--arena-accent-red);opacity:0.85",
                            "Danger zone"
                        }
                    }
                    div { class: "sga-danger-row",
                        div { class: "sga-danger-row__text",
                            div { class: "sga-danger-row__title", "{tr.delete_space}" }
                            div { class: "sga-danger-row__sub",
                                "Permanently removes the space, all actions, and comments. Cannot be undone."
                            }
                        }
                        button {
                            r#type: "button",
                            class: "sga-btn sga-btn--danger",
                            disabled: delete_space_action.pending(),
                            onclick: move |_| {
                                // Open a confirmation popup before
                                // firing the mutation. Confirm closes
                                // the popup then triggers the action
                                // (which navigates home on success);
                                // cancel just closes.
                                let on_confirm = move |_| {
                                    popup.close();
                                    delete_space_action.call();
                                };
                                let on_cancel = move |_| popup.close();
                                popup
                                    .open(rsx! {
                                    DeleteSpacePopup { on_confirm, on_cancel }
                                });
                            },
                            if delete_space_action.pending() {
                                {tr.deleting}
                            } else {
                                {tr.delete_space}
                            }
                        }
                    }
                }
            }

            // ── Sticky footer (autosave indicator) ──────
            footer { class: "sga-footer",
                div { class: "sga-footer__left",
                    span { "Changes auto-save" }
                    span { class: "sga-footer__pill",
                        span { style: "width:6px;height:6px;border-radius:50%;background:var(--arena-accent-teal);display:inline-block" }
                        "Synced"
                    }
                }
            }
        }
    }
}
