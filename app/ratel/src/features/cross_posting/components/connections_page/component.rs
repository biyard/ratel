use crate::common::*;
use crate::features::cross_posting::components::bluesky_connect_modal::BlueskyConnectModal;
use crate::features::cross_posting::hooks::{use_cross_posting_provider, UseCrossPosting};
use crate::features::cross_posting::i18n::ConnectionsPageTranslate;
use crate::features::cross_posting::models::ConnectionStatus;
use crate::features::cross_posting::types::{ConnectionResponse, SocialPlatform};

/// Settings → Connections (`/{username}/settings/connections`).
///
/// Phase 1A: Bluesky is the only fully-wired platform. LinkedIn, Threads,
/// and Farcaster render as static "Coming soon" cards. The connect /
/// disconnect / auto-post-toggle actions all flow through `async fn`
/// methods on the `UseCrossPosting` controller installed at the page root.
#[component]
pub fn ConnectionsPage(username: String) -> Element {
    let _ = username; // Currently unused — controller scopes by session user_pk
    let mut cp = use_cross_posting_provider()?;
    let UseCrossPosting {
        connections,
        connected_count,
        posts_this_month,
        ..
    } = cp;

    let mut modal_open = use_signal(|| false);
    let mut toast = use_toast();
    let nav = use_navigator();
    let t: ConnectionsPageTranslate = use_translate();

    let conn_list: Vec<ConnectionResponse> = connections();
    let bsky: Option<ConnectionResponse> = conn_list
        .iter()
        .find(|c| c.platform == SocialPlatform::Bluesky)
        .cloned();
    let bsky_connected = bsky
        .as_ref()
        .map(|c| c.status == ConnectionStatus::Connected)
        .unwrap_or(false);

    rsx! {
        SeoMeta { title: "{t.title}" }

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

        div { class: "connections-arena",
            // ── Top bar ────────────────────────────────────────────
            header { class: "arena-topbar",
                div { class: "arena-topbar__left",
                    button {
                        class: "back-btn",
                        "aria-label": "{t.btn_back_aria}",
                        onclick: move |_| {
                            nav.go_back();
                        },
                        svg {
                            "viewBox": "0 0 24 24",
                            "fill": "none",
                            "stroke": "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            path { "d": "M19 12H5" }
                            path { "d": "M12 19l-7-7 7-7" }
                        }
                    }
                    div { class: "topbar-title",
                        span { class: "topbar-title__eyebrow", "{t.eyebrow}" }
                        span { class: "topbar-title__main", "{t.title}" }
                    }
                }
            }

            main { class: "connections-page",
                // ── Hero ────────────────────────────────────────────
                section { class: "connections-hero",
                    div { class: "connections-hero__main",
                        span { class: "connections-hero__eyebrow",
                            svg {
                                "viewBox": "0 0 24 24",
                                "fill": "currentColor",
                                path { "d": "M12 10.5c-1.3-2.5-4.9-7.2-8.2-9.5C.7-1.2 0 .5 0 1.5c0 1.1.6 9.1 1 10.3.8 4 5.5 5.1 9.6 4.4-6.5 1.1-12.3 3.5-4.7 11.4 2.5 2.6 3.5-2.6 4-5.2.5 2.6 1.6 7.8 4 5.2 7.7-7.9 1.9-10.4-4.6-11.5 4 .7 8.8-.4 9.6-4.4.4-1.2 1-9.2 1-10.3 0-1-.7-2.7-3.8-.5-3.3 2.3-6.9 7-8.2 9.6z" }
                            }
                            "{t.hero_eyebrow}"
                        }
                        h1 { class: "connections-hero__title",
                            "{t.hero_title} "
                            span { class: "connections-hero__title-accent", "{t.hero_title_accent}" }
                        }
                        p { class: "connections-hero__sub", "{t.hero_sub}" }
                        div { class: "connections-hero__stats",
                            div { class: "connections-hero__stat",
                                span { class: "connections-hero__stat-value connections-hero__stat-value--accent",
                                    "{connected_count()}"
                                }
                                span { class: "connections-hero__stat-label", "{t.stat_connected}" }
                            }
                            div { class: "connections-hero__stat",
                                span { class: "connections-hero__stat-value", "{posts_this_month()}" }
                                span { class: "connections-hero__stat-label", "{t.stat_this_month}" }
                            }
                        }
                    }
                    div {
                        class: "connections-hero__orbit",
                        "aria-hidden": "true",
                        div { class: "connections-hero__orbit-ring" }
                        div { class: "connections-hero__orbit-ring connections-hero__orbit-ring--inner" }
                        div { class: "connections-hero__orbit-core", "RATEL" }
                        div { class: "connections-hero__orbit-chip connections-hero__orbit-chip--bsky",
                            svg {
                                "viewBox": "0 0 24 24",
                                "fill": "currentColor",
                                path { "d": "M12 10.5c-1.3-2.5-4.9-7.2-8.2-9.5C.7-1.2 0 .5 0 1.5c0 1.1.6 9.1 1 10.3.8 4 5.5 5.1 9.6 4.4-6.5 1.1-12.3 3.5-4.7 11.4 2.5 2.6 3.5-2.6 4-5.2.5 2.6 1.6 7.8 4 5.2 7.7-7.9 1.9-10.4-4.6-11.5 4 .7 8.8-.4 9.6-4.4.4-1.2 1-9.2 1-10.3 0-1-.7-2.7-3.8-.5-3.3 2.3-6.9 7-8.2 9.6z" }
                            }
                        }
                        div { class: "connections-hero__orbit-chip connections-hero__orbit-chip--linkedin",
                            svg {
                                "viewBox": "0 0 24 24",
                                "fill": "currentColor",
                                path { "d": "M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.063 2.063 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" }
                            }
                        }
                        div { class: "connections-hero__orbit-chip connections-hero__orbit-chip--threads",
                            svg {
                                "viewBox": "0 0 24 24",
                                "fill": "currentColor",
                                path {
                                    "d": "M12.186 24h-.007c-3.581-.024-6.334-1.205-8.184-3.509C2.35 18.44 1.5 15.586 1.472 12.01v-.017c.03-3.579.879-6.43 2.525-8.482C5.845 1.205 8.6.024 12.18 0h.014c2.746.02 5.043.725 6.826 2.098 1.677 1.29 2.858 3.13 3.509 5.467l-2.04.569c-1.104-3.96-3.898-5.984-8.304-6.015-2.91.022-5.11.936-6.54 2.717C4.307 6.504 3.616 8.914 3.589 12c.027 3.086.718 5.496 2.057 7.164 1.43 1.783 3.631 2.698 6.54 2.717 2.623-.02 4.358-.631 5.8-2.045 1.647-1.613 1.618-3.593 1.09-4.798-.31-.71-.873-1.3-1.634-1.75-.192 1.352-.622 2.446-1.284 3.272-.886 1.102-2.14 1.704-3.73 1.79-1.202.065-2.361-.218-3.259-.801-1.063-.689-1.685-1.74-1.752-2.964-.065-1.19.408-2.285 1.33-3.082.88-.76 2.119-1.207 3.583-1.291a13.853 13.853 0 013.02.142c-.126-.742-.375-1.332-.75-1.757-.513-.586-1.308-.883-2.359-.89h-.029c-.844 0-1.992.232-2.721 1.32L9.136 8.723c.976-1.45 2.561-2.249 4.464-2.249h.042c3.186.02 5.083 1.965 5.276 5.35.108.046.216.094.324.14 1.61.664 2.757 1.733 3.304 3.095 1.03 2.54.63 5.43-1.192 7.194-1.923 1.865-4.24 2.742-7.168 2.742v.005z",
                                }
                            }
                        }
                    }
                }

                // ── Section heading ────────────────────────────────
                div { class: "connections-section-head",
                    span { class: "connections-section-head__title", "{t.section_platforms}" }
                    span { class: "connections-section-head__meta", "{t.section_meta_phase1}" }
                }

                // ── Platform cards ─────────────────────────────────
                div { class: "platforms",
                    // Bluesky — Phase 1A active
                    article {
                        class: "plat",
                        "data-platform": "bsky",
                        "data-connected": "{bsky_connected}",
                        div { class: "plat__body",
                            span { class: "plat__logo plat__logo--bsky",
                                svg {
                                    "viewBox": "0 0 24 24",
                                    "fill": "currentColor",
                                    path { "d": "M12 10.5c-1.3-2.5-4.9-7.2-8.2-9.5C.7-1.2 0 .5 0 1.5c0 1.1.6 9.1 1 10.3.8 4 5.5 5.1 9.6 4.4-6.5 1.1-12.3 3.5-4.7 11.4 2.5 2.6 3.5-2.6 4-5.2.5 2.6 1.6 7.8 4 5.2 7.7-7.9 1.9-10.4-4.6-11.5 4 .7 8.8-.4 9.6-4.4.4-1.2 1-9.2 1-10.3 0-1-.7-2.7-3.8-.5-3.3 2.3-6.9 7-8.2 9.6z" }
                                }
                            }
                            div { class: "plat__main",
                                div { class: "plat__name-row",
                                    span { class: "plat__name", "{t.bluesky_name}" }
                                    if bsky_connected {
                                        span { class: "status-pill status-pill--connected",
                                            "{t.status_connected}"
                                        }
                                    } else {
                                        span { class: "status-pill status-pill--off",
                                            "{t.status_not_connected}"
                                        }
                                    }
                                    span { class: "plat__limit", "{t.bluesky_limit}" }
                                }
                                div { class: "plat__handle",
                                    if let Some(c) = bsky.clone() {
                                        span { "@{c.external_handle}" }
                                    } else {
                                        span { "{t.bluesky_subtitle_default}" }
                                    }
                                }
                            }
                            div { class: "plat__actions",
                                if bsky_connected {
                                    button {
                                        class: "connections-btn connections-btn--ghost",
                                        onclick: move |_| async move {
                                            if let Err(e) = cp.disconnect(SocialPlatform::Bluesky).await {
                                                toast.error(e);
                                            }
                                        },
                                        "{t.btn_disconnect}"
                                    }
                                } else {
                                    button {
                                        class: "connections-btn connections-btn--connect-bsky",
                                        onclick: move |_| modal_open.set(true),
                                        "{t.btn_connect}"
                                    }
                                }
                            }
                        }
                        if let Some(c) = bsky.clone() {
                            if c.status == ConnectionStatus::Connected {
                                {
                                    let auto_post = c.auto_post_enabled;
                                    rsx! {
                                        div { class: "plat__subrow",
                                            div { class: "plat__subrow-item",
                                                strong { "{c.posts_syndicated_count}" }
                                                span { "{t.posts_syndicated_count_label}" }
                                            }
                                            span { class: "plat__subrow-sep" }
                                            div { class: "plat__subrow-item",
                                                span { "{t.auto_post}" }
                                                button {
                                                    class: "switch",
                                                    "aria-checked": "{auto_post}",
                                                    "aria-label": "{t.auto_post}",
                                                    onclick: move |_| async move {
                                                        if let Err(e) = cp.toggle_auto_post(SocialPlatform::Bluesky, !auto_post).await {
                                                            toast.error(e);
                                                        }
                                                    },
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // LinkedIn — coming in Phase 1B
                    article {
                        class: "plat",
                        "data-platform": "linkedin",
                        "data-soon": "true",
                        div { class: "plat__body",
                            span { class: "plat__logo plat__logo--linkedin",
                                svg {
                                    "viewBox": "0 0 24 24",
                                    "fill": "currentColor",
                                    path { "d": "M20.447 20.452h-3.554v-5.569c0-1.328-.027-3.037-1.852-3.037-1.853 0-2.136 1.445-2.136 2.939v5.667H9.351V9h3.414v1.561h.046c.477-.9 1.637-1.85 3.37-1.85 3.601 0 4.267 2.37 4.267 5.455v6.286zM5.337 7.433a2.062 2.062 0 01-2.063-2.065 2.063 2.063 0 112.063 2.065zm1.782 13.019H3.555V9h3.564v11.452zM22.225 0H1.771C.792 0 0 .774 0 1.729v20.542C0 23.227.792 24 1.771 24h20.451C23.2 24 24 23.227 24 22.271V1.729C24 .774 23.2 0 22.222 0h.003z" }
                                }
                            }
                            div { class: "plat__main",
                                div { class: "plat__name-row",
                                    span { class: "plat__name", "{t.linkedin_name}" }
                                    span { class: "status-pill status-pill--soon",
                                        "{t.status_coming_soon}"
                                    }
                                    span { class: "plat__limit", "{t.linkedin_limit}" }
                                }
                                div { class: "plat__handle",
                                    span { "{t.linkedin_subtitle}" }
                                }
                            }
                            div { class: "plat__actions",
                                button {
                                    class: "connections-btn connections-btn--ghost",
                                    disabled: true,
                                    "{t.btn_notify}"
                                }
                            }
                        }
                    }

                    // Threads — coming in Phase 1C
                    article {
                        class: "plat",
                        "data-platform": "threads",
                        "data-soon": "true",
                        div { class: "plat__body",
                            span { class: "plat__logo plat__logo--threads",
                                svg {
                                    "viewBox": "0 0 24 24",
                                    "fill": "currentColor",
                                    path {
                                        "d": "M12.186 24h-.007c-3.581-.024-6.334-1.205-8.184-3.509C2.35 18.44 1.5 15.586 1.472 12.01v-.017c.03-3.579.879-6.43 2.525-8.482C5.845 1.205 8.6.024 12.18 0h.014c2.746.02 5.043.725 6.826 2.098 1.677 1.29 2.858 3.13 3.509 5.467l-2.04.569c-1.104-3.96-3.898-5.984-8.304-6.015-2.91.022-5.11.936-6.54 2.717C4.307 6.504 3.616 8.914 3.589 12c.027 3.086.718 5.496 2.057 7.164 1.43 1.783 3.631 2.698 6.54 2.717 2.623-.02 4.358-.631 5.8-2.045 1.647-1.613 1.618-3.593 1.09-4.798-.31-.71-.873-1.3-1.634-1.75-.192 1.352-.622 2.446-1.284 3.272-.886 1.102-2.14 1.704-3.73 1.79-1.202.065-2.361-.218-3.259-.801-1.063-.689-1.685-1.74-1.752-2.964-.065-1.19.408-2.285 1.33-3.082.88-.76 2.119-1.207 3.583-1.291a13.853 13.853 0 013.02.142c-.126-.742-.375-1.332-.75-1.757-.513-.586-1.308-.883-2.359-.89h-.029c-.844 0-1.992.232-2.721 1.32L9.136 8.723c.976-1.45 2.561-2.249 4.464-2.249h.042c3.186.02 5.083 1.965 5.276 5.35.108.046.216.094.324.14 1.61.664 2.757 1.733 3.304 3.095 1.03 2.54.63 5.43-1.192 7.194-1.923 1.865-4.24 2.742-7.168 2.742v.005z",
                                    }
                                }
                            }
                            div { class: "plat__main",
                                div { class: "plat__name-row",
                                    span { class: "plat__name", "{t.threads_name}" }
                                    span { class: "status-pill status-pill--soon",
                                        "{t.status_coming_soon}"
                                    }
                                    span { class: "plat__limit", "{t.threads_limit}" }
                                }
                                div { class: "plat__handle",
                                    span { "{t.threads_subtitle}" }
                                }
                            }
                            div { class: "plat__actions",
                                button {
                                    class: "connections-btn connections-btn--ghost",
                                    disabled: true,
                                    "{t.btn_notify}"
                                }
                            }
                        }
                    }

                    // Farcaster — Phase 2
                    article {
                        class: "plat",
                        "data-platform": "farcaster",
                        "data-soon": "true",
                        div { class: "plat__body",
                            span { class: "plat__logo plat__logo--farcaster",
                                svg {
                                    "viewBox": "0 0 225 225",
                                    "fill": "currentColor",
                                    path { "d": "M59 34h107v157h-15v-72h-.1c-1.7-18.5-17.3-33-36.3-33S78.9 100.5 77.1 119H77v72H59V34z" }
                                    path { "d": "M28 56l6 22h6v91c-3 0-5 2-5 5v7h-1c-3 0-5 2-5 5v7h57v-7c0-3-2-5-5-5h-1v-7c0-3-2-5-5-5h-7v-91-22H28z" }
                                    path { "d": "M145 169c-3 0-5 2-5 5v7h-1c-3 0-5 2-5 5v7h57v-7c0-3-2-5-5-5h-1v-7c0-3-2-5-5-5v-91l6-22h-40v22 91h-1z" }
                                }
                            }
                            div { class: "plat__main",
                                div { class: "plat__name-row",
                                    span { class: "plat__name", "{t.farcaster_name}" }
                                    span { class: "status-pill status-pill--soon", "{t.status_phase2}" }
                                    span { class: "plat__limit", "{t.farcaster_limit}" }
                                }
                                div { class: "plat__handle",
                                    span { "{t.farcaster_subtitle}" }
                                }
                            }
                            div { class: "plat__actions",
                                button {
                                    class: "connections-btn connections-btn--ghost",
                                    disabled: true,
                                    "{t.btn_notify}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
