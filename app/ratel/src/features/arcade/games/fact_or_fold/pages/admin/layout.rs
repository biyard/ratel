use crate::common::*;
use crate::features::arcade::games::fact_or_fold::pages::admin::FactFoldAdminLayoutTranslate;
use crate::route::Route;

/// Sub-layout for `/admin/fact-or-fold/*` pages. Adds the
/// arcade-themed brand row + tab navigation between the 5 admin
/// surfaces (Headlines, Schedule, Stats, Reports, Settings) so each
/// page module can render only its own content.
#[component]
pub fn FactFoldAdminLayout() -> Element {
    let tr: FactFoldAdminLayoutTranslate = use_translate();
    let route: Route = use_route();

    // Hoist Route values so dx fmt doesn't try to split the empty
    // struct literals across lines (which historically dropped the
    // closing `);` of the surrounding call).
    let r_headlines = Route::FactFoldAdminHeadlinesPage {};
    let r_schedule = Route::FactFoldAdminSchedulePage {};
    let r_stats = Route::FactFoldAdminStatsPage {};
    let r_reports = Route::FactFoldAdminReportsPage {};
    let r_settings = Route::FactFoldAdminSettingsPage {};
    let r_new = Route::FactFoldAdminNewHeadlinePage {};

    rsx! {
        div { class: "ff-admin-arena",
            header { class: "ff-admin-arena__topbar",
                div { class: "ff-admin-arena__brand",
                    div { class: "ff-admin-arena__brand-logo", "R" }
                    div { class: "ff-admin-arena__brand-text",
                        div { class: "ff-admin-arena__brand-name", "{tr.brand}" }
                        div { class: "ff-admin-arena__brand-sub", "{tr.brand_sub}" }
                    }
                }
                nav { class: "ff-admin-arena__tabs", role: "tablist",
                    Link {
                        class: "ff-admin-arena__tab",
                        "aria-selected": route == r_headlines,
                        to: r_headlines.clone(),
                        "{tr.tab_headlines}"
                    }
                    Link {
                        class: "ff-admin-arena__tab",
                        "aria-selected": route == r_schedule,
                        to: r_schedule.clone(),
                        "{tr.tab_schedule}"
                    }
                    Link {
                        class: "ff-admin-arena__tab",
                        "aria-selected": route == r_stats,
                        to: r_stats.clone(),
                        "{tr.tab_stats}"
                    }
                    Link {
                        class: "ff-admin-arena__tab",
                        "aria-selected": route == r_reports,
                        to: r_reports.clone(),
                        "{tr.tab_reports}"
                    }
                    Link {
                        class: "ff-admin-arena__tab",
                        "aria-selected": route == r_settings,
                        to: r_settings.clone(),
                        "{tr.tab_settings}"
                    }
                }
                div { class: "ff-admin-arena__cta",
                    Link { class: "ff-admin-arena__new-btn", to: r_new,
                        span { class: "ff-admin-arena__new-icon", "✚" }
                        span { "{tr.new_headline_cta}" }
                    }
                }
            }
            main { class: "ff-admin-arena__main", Outlet::<Route> {} }
        }
    }
}
