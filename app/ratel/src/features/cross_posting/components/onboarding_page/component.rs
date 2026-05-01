use crate::common::*;
use crate::features::auth::controllers::mark_onboarding_seen_handler;
use crate::features::auth::hooks::use_user_context;
use crate::features::cross_posting::components::bluesky_connect_modal::BlueskyConnectModal;
use crate::features::cross_posting::hooks::{use_cross_posting_provider, UseCrossPosting};
use crate::features::cross_posting::i18n::OnboardingPageTranslate;
use crate::features::cross_posting::models::ConnectionStatus;
use crate::features::cross_posting::types::{ConnectionResponse, SocialPlatform};

/// Post-signup "Connect your networks" interstitial (`/onboarding/connections`).
///
/// FR-2 (#8–#14) — single screen, shown once per account between signup-success
/// and home. Continue / Skip / any successful connect flips
/// `User.interstitial_seen=true` server-side (FR-2 #13) and routes the user
/// home; AC-4b ensures it never re-shows after that.
#[component]
pub fn OnboardingPage() -> Element {
    let mut cp = use_cross_posting_provider()?;
    let UseCrossPosting { connections, .. } = cp;

    let mut user_ctx = use_user_context();
    let nav = use_navigator();
    let mut toast = use_toast();
    let mut modal_open = use_signal(|| false);
    let t: OnboardingPageTranslate = use_translate();

    // AC-4b — if the user has already dismissed this once, never auto-show
    // it again. A direct URL hit lands them straight on home instead.
    use_effect(move || {
        let already_seen = user_ctx()
            .user
            .as_ref()
            .map(|u| u.interstitial_seen)
            .unwrap_or(false);
        if already_seen {
            nav.replace(crate::Route::Index {});
        }
    });

    let conn_list: Vec<ConnectionResponse> = connections();
    let bsky: Option<ConnectionResponse> = conn_list
        .iter()
        .find(|c| c.platform == SocialPlatform::Bluesky)
        .cloned();
    let bsky_connected = bsky
        .as_ref()
        .map(|c| c.status == ConnectionStatus::Connected)
        .unwrap_or(false);

    // FR-2 #13 — flip the seen flag and land on home. Failure to update the
    // flag is non-fatal (the user can always re-trigger onboarding from
    // Settings); we toast the error but still navigate so the user isn't
    // stuck on this screen.
    let dismiss_and_continue = move || async move {
        match mark_onboarding_seen_handler().await {
            Ok(updated_user) => {
                let mut ctx = user_ctx();
                ctx.user = Some(updated_user);
                user_ctx.set(ctx);
            }
            Err(e) => {
                tracing::warn!(error = %e, "mark_onboarding_seen failed; continuing anyway");
                toast.error(e);
            }
        }
        nav.replace(crate::Route::Index {});
    };

    rsx! {
        SeoMeta { title: "{t.seo_title}" }

        BlueskyConnectModal {
            open: modal_open,
            on_submit: move |(handle, app_password): (String, String)| async move {
                if let Err(e) = cp.connect_bluesky(handle, app_password).await {
                    toast.error(e);
                    return;
                }
                modal_open.set(false);
            },
        }

        div { class: "onboarding-arena",
            // Top bar — Skip-for-now link as fallback exit.
            header { class: "onboarding-topbar",
                div { class: "onboarding-brand",
                    span { class: "onboarding-brand__dot", "R" }
                    "Ratel"
                }
                button {
                    class: "onboarding-topbar__skip",
                    "data-testid": "onboarding-skip-link",
                    onclick: move |_| async move { dismiss_and_continue().await },
                    "{t.topbar_skip}"
                }
            }

            main { class: "onboarding-page",
                div { class: "onboarding-wizard",
                    span { class: "onboarding-wizard__eyebrow",
                        svg {
                            "viewBox": "0 0 24 24",
                            "fill": "none",
                            "stroke": "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            circle { "cx": "18", "cy": "5", "r": "3" }
                            circle { "cx": "6", "cy": "12", "r": "3" }
                            circle { "cx": "18", "cy": "19", "r": "3" }
                            line {
                                "x1": "8.59",
                                "y1": "13.51",
                                "x2": "15.42",
                                "y2": "17.49",
                            }
                            line {
                                "x1": "15.41",
                                "y1": "6.51",
                                "x2": "8.59",
                                "y2": "10.49",
                            }
                        }
                        "{t.eyebrow}"
                    }

                    h1 { class: "onboarding-wizard__title",
                        "{t.title_lead}"
                        br {}
                        span { class: "onboarding-wizard__title-accent", "{t.title_accent}" }
                    }

                    p { class: "onboarding-wizard__sub", "{t.sub}" }

                    // ── Platform rows ────────────────────────────────
                    div { class: "onboarding-grid",
                        // Bluesky — Phase 1A active
                        article {
                            class: "onboarding-row",
                            "data-platform": "bsky",
                            "data-connected": "{bsky_connected}",
                            span { class: "onboarding-logo onboarding-logo--bsky",
                                svg {
                                    "viewBox": "0 0 24 24",
                                    "fill": "currentColor",
                                    path { "d": "M12 10.5c-1.3-2.5-4.9-7.2-8.2-9.5C.7-1.2 0 .5 0 1.5c0 1.1.6 9.1 1 10.3.8 4 5.5 5.1 9.6 4.4-6.5 1.1-12.3 3.5-4.7 11.4 2.5 2.6 3.5-2.6 4-5.2.5 2.6 1.6 7.8 4 5.2 7.7-7.9 1.9-10.4-4.6-11.5 4 .7 8.8-.4 9.6-4.4.4-1.2 1-9.2 1-10.3 0-1-.7-2.7-3.8-.5-3.3 2.3-6.9 7-8.2 9.6z" }
                                }
                            }
                            div { class: "onboarding-body",
                                div { class: "onboarding-name", "Bluesky" }
                                div { class: "onboarding-meta",
                                    if let Some(c) = bsky.as_ref() {
                                        if c.status == ConnectionStatus::Connected {
                                            strong { "@{c.external_handle}" }
                                            "{t.bluesky_meta_connected_suffix}"
                                        } else {
                                            "{t.bluesky_meta_default}"
                                        }
                                    } else {
                                        "{t.bluesky_meta_default}"
                                    }
                                }
                            }
                            div { class: "onboarding-action",
                                if bsky_connected {
                                    span { class: "onboarding-connected-badge",
                                        svg {
                                            "viewBox": "0 0 24 24",
                                            "fill": "none",
                                            "stroke": "currentColor",
                                            "stroke-width": "3",
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            polyline { "points": "20 6 9 17 4 12" }
                                        }
                                        "{t.status_connected}"
                                    }
                                } else {
                                    button {
                                        class: "onboarding-btn-connect onboarding-btn-connect--bsky",
                                        "data-testid": "onboarding-connect-bluesky",
                                        onclick: move |_| modal_open.set(true),
                                        svg {
                                            "viewBox": "0 0 24 24",
                                            "fill": "none",
                                            "stroke": "currentColor",
                                            "stroke-width": "2.5",
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            polyline { "points": "5 12 19 12" }
                                            polyline { "points": "12 5 19 12 12 19" }
                                        }
                                        "{t.btn_connect}"
                                    }
                                }
                            }
                        }

                        // LinkedIn — Phase 1B placeholder
                        article {
                            class: "onboarding-row",
                            "data-platform": "linkedin",
                            "data-connected": "false",
                            "data-soon": "true",
                            span { class: "onboarding-logo onboarding-logo--linkedin",
                                svg {
                                    "viewBox": "0 0 24 24",
                                    "fill": "currentColor",
                                    path { "d": "M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.063 2.063 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" }
                                }
                            }
                            div { class: "onboarding-body",
                                div { class: "onboarding-name", "LinkedIn" }
                                div { class: "onboarding-meta", "{t.linkedin_meta}" }
                            }
                            div { class: "onboarding-action",
                                button {
                                    class: "onboarding-btn-connect onboarding-btn-connect--soon",
                                    disabled: true,
                                    "{t.coming_soon}"
                                }
                            }
                        }

                        // Threads — Phase 1C placeholder
                        article {
                            class: "onboarding-row",
                            "data-platform": "threads",
                            "data-connected": "false",
                            "data-soon": "true",
                            span { class: "onboarding-logo onboarding-logo--threads",
                                svg {
                                    "viewBox": "0 0 24 24",
                                    "fill": "currentColor",
                                    path { "d": "M12.186 24h-.007c-3.581-.024-6.334-1.205-8.184-3.509C2.35 18.44 1.5 15.586 1.472 12.01v-.017c.03-3.579.879-6.43 2.525-8.482C5.845 1.205 8.6.024 12.18 0h.014c2.746.02 5.043.725 6.826 2.098 1.677 1.29 2.858 3.13 3.509 5.467l-2.04.569c-1.104-3.96-3.898-5.984-8.304-6.015-2.91.022-5.11.936-6.54 2.717C4.307 6.504 3.616 8.914 3.589 12c.027 3.086.718 5.496 2.057 7.164 1.43 1.783 3.631 2.698 6.54 2.717 2.623-.02 4.358-.631 5.8-2.045 1.647-1.613 1.618-3.593 1.09-4.798z" }
                                }
                            }
                            div { class: "onboarding-body",
                                div { class: "onboarding-name", "Threads" }
                                div { class: "onboarding-meta", "{t.threads_meta}" }
                            }
                            div { class: "onboarding-action",
                                button {
                                    class: "onboarding-btn-connect onboarding-btn-connect--soon",
                                    disabled: true,
                                    "{t.coming_soon}"
                                }
                            }
                        }
                    }

                    // ── Benefits card ────────────────────────────────
                    div { class: "onboarding-benefits",
                        div { class: "onboarding-benefit",
                            span { class: "onboarding-benefit__icon",
                                svg {
                                    "viewBox": "0 0 24 24",
                                    "fill": "none",
                                    "stroke": "currentColor",
                                    "stroke-width": "2",
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    circle { "cx": "18", "cy": "5", "r": "3" }
                                    circle { "cx": "6", "cy": "12", "r": "3" }
                                    circle { "cx": "18", "cy": "19", "r": "3" }
                                    line {
                                        "x1": "8.59",
                                        "y1": "13.51",
                                        "x2": "15.42",
                                        "y2": "17.49",
                                    }
                                    line {
                                        "x1": "15.41",
                                        "y1": "6.51",
                                        "x2": "8.59",
                                        "y2": "10.49",
                                    }
                                }
                            }
                            span { class: "onboarding-benefit__label", "{t.benefit_auto_label}" }
                            span { class: "onboarding-benefit__hint", "{t.benefit_auto_hint}" }
                        }
                        div { class: "onboarding-benefit",
                            span { class: "onboarding-benefit__icon",
                                svg {
                                    "viewBox": "0 0 24 24",
                                    "fill": "none",
                                    "stroke": "currentColor",
                                    "stroke-width": "2",
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    path { "d": "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                                }
                            }
                            span { class: "onboarding-benefit__label", "{t.benefit_backlinks_label}" }
                            span { class: "onboarding-benefit__hint", "{t.benefit_backlinks_hint}" }
                        }
                        div { class: "onboarding-benefit",
                            span { class: "onboarding-benefit__icon",
                                svg {
                                    "viewBox": "0 0 24 24",
                                    "fill": "none",
                                    "stroke": "currentColor",
                                    "stroke-width": "2",
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    rect {
                                        "x": "3",
                                        "y": "11",
                                        "width": "18",
                                        "height": "11",
                                        "rx": "2",
                                        "ry": "2",
                                    }
                                    path { "d": "M7 11V7a5 5 0 0 1 10 0v4" }
                                }
                            }
                            span { class: "onboarding-benefit__label", "{t.benefit_secure_label}" }
                            span { class: "onboarding-benefit__hint", "{t.benefit_secure_hint}" }
                        }
                    }

                    // ── CTA row ──────────────────────────────────────
                    div { class: "onboarding-cta-row",
                        button {
                            class: "onboarding-cta onboarding-cta--ghost",
                            "data-testid": "onboarding-skip",
                            onclick: move |_| async move { dismiss_and_continue().await },
                            "{t.cta_skip}"
                        }
                        button {
                            class: "onboarding-cta onboarding-cta--primary",
                            "data-testid": "onboarding-continue",
                            onclick: move |_| async move { dismiss_and_continue().await },
                            "{t.cta_continue}"
                            svg {
                                "viewBox": "0 0 24 24",
                                "fill": "none",
                                "stroke": "currentColor",
                                "stroke-width": "2.5",
                                "stroke-linecap": "round",
                                "stroke-linejoin": "round",
                                polyline { "points": "5 12 19 12" }
                                polyline { "points": "12 5 19 12 12 19" }
                            }
                        }
                    }

                    p { class: "onboarding-footer-note",
                        strong { "{t.footer_note_pro_tip}" }
                        " {t.footer_note_body}"
                    }
                }
            }
        }
    }
}

