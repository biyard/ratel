use super::*;
use crate::features::spaces::space_common::providers::use_space_context;

mod attribute_groups;
mod collective_panel;
mod conditional_table;
mod total_quota;
mod viewer;

use attribute_groups::*;
use collective_panel::*;
use conditional_table::*;
use total_quota::*;
use viewer::*;

const DEFAULT_SPACE_LOGO: &str = "https://metadata.ratel.foundation/logos/logo-symbol.png";

/// Public entrypoint — dispatches between the Creator arena view and
/// the viewer fallback based on the *real* space role.
///
/// Uses `ctx.role()` (the underlying Loader value) instead of
/// `use_space_role()` (which exposes the `current_role` memo). The
/// memo flips Creator → Participant once the space is Ongoing so the
/// creator can preview the participant view; that preview toggle
/// must not affect admin-surface gating like the panels composer.
#[component]
pub fn SpacePanelsAppPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let mut ctx = use_space_context();
    let real_role = ctx.role();

    if real_role == SpaceUserRole::Creator {
        rsx! {
            CreatorArenaPage { space_id }
        }
    } else {
        rsx! {
            ViewerPage { space_id }
        }
    }
}

/// Creator-only arena view. Removes the mockup's Save/Cancel buttons
/// (both topbar and footer) — the `UseSpacePanels` hook auto-saves every
/// mutation. The footer keeps the "Changes saved" pill so the user has a
/// visible confirmation after each auto-save.
#[component]
fn CreatorArenaPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: PanelsTranslate = use_translate();
    let space = use_space();
    let nav = use_navigator();

    // Instantiate the controller hook once at the page root. Section
    // components re-enter the same cached instance via `use_space_panels`.
    let UseSpacePanels { panels, .. } = use_space_panels(space_id)?;

    let panel_list = panels.read().clone();
    let is_saving = panels.loading();

    let has_collective = is_collective_option(PanelOption::University, &panel_list)
        || is_collective_option(PanelOption::Age, &panel_list)
        || is_collective_option(PanelOption::Gender, &panel_list);

    let has_conditional = is_conditional_option(PanelOption::Age, &panel_list)
        || is_conditional_option(PanelOption::Gender, &panel_list);

    let space_data = space();
    let space_logo = if space_data.logo.is_empty() {
        DEFAULT_SPACE_LOGO.to_string()
    } else {
        space_data.logo.clone()
    };
    let space_title = space_data.title.clone();

    rsx! {

        div { class: "space-panels-arena",
            // ── Arena topbar (no Save/Cancel/Settings — auto-save) ───
            header { class: "spa-topbar", role: "banner",
                div { class: "spa-topbar__left",
                    button {
                        r#type: "button",
                        class: "spa-back-btn",
                        "aria-label": "{tr.back_aria}",
                        "data-testid": "topbar-back",
                        onclick: move |_| {
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
                        class: "spa-topbar__logo",
                        alt: "Space logo",
                        src: "{space_logo}",
                    }
                    nav { class: "spa-breadcrumb",
                        span { class: "spa-breadcrumb__item", "{space_title}" }
                        span { class: "spa-breadcrumb__sep", "›" }
                        span { class: "spa-breadcrumb__item", "{tr.breadcrumb_apps}" }
                        span { class: "spa-breadcrumb__sep", "›" }
                        span { class: "spa-breadcrumb__item spa-breadcrumb__current",
                            "{tr.breadcrumb_panels}"
                        }
                    }
                    span { class: "spa-type-badge", "data-testid": "type-badge",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            path { d: "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" }
                            circle { cx: "9", cy: "7", r: "4" }
                            path { d: "M23 21v-2a4 4 0 0 0-3-3.87M16 3.13a4 4 0 0 1 0 7.75" }
                        }
                        "{tr.type_badge}"
                    }
                    span { class: "spa-topbar-title", "{tr.topbar_title}" }
                }
            }

            // ── Main body ───────────────────────────────────────────
            main { class: "spa-body",
                TotalQuota { space_id }
                AttributeGroupsSection { space_id }

                if has_collective {
                    CollectivePanelSection { space_id }
                }

                if has_conditional {
                    ConditionalTableSection { space_id }
                }
            }

            // ── Sticky footer (Saved pill only) ─────────────────────
            footer { class: "spa-footer",
                div { class: "spa-footer__left",
                    if is_saving {
                        span {
                            class: "spa-footer__pill spa-footer__pill--saving",
                            "data-testid": "autosave-pill",
                            span { class: "spa-footer__pill-dot" }
                            "{tr.footer_saving}"
                        }
                    } else {
                        span {
                            class: "spa-footer__pill",
                            "data-testid": "autosave-pill",
                            span { class: "spa-footer__pill-dot" }
                            "{tr.footer_saved}"
                        }
                    }
                }
            }
        }
    }
}
