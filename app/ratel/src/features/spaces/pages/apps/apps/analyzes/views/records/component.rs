use crate::features::spaces::pages::apps::apps::analyzes::*;
use crate::features::spaces::space_common::hooks::use_space;
use crate::*;

const DEFAULT_SPACE_LOGO: &str = "https://metadata.ratel.foundation/logos/logo-symbol.png";

#[component]
pub fn SpaceAnalyzeRecordsPage(
    space_id: ReadSignal<SpacePartition>,
    report_id: ReadSignal<String>,
) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let space = use_space();
    let nav = use_navigator();
    let mut ctrl = use_analyze_records(space_id, report_id)?;

    let report_payload = ctrl.report.read().clone();
    let report = report_payload.report;
    let space_data = space();
    let space_logo = if space_data.logo.is_empty() {
        DEFAULT_SPACE_LOGO.to_string()
    } else {
        space_data.logo.clone()
    };
    let space_title = space_data.title.clone();
    let report_name = report.name.clone();
    let filters = report.filters.clone();
    let selected_filter = *ctrl.selected_filter.read();
    let records = ctrl.records.items();
    let has_more = ctrl.records.has_more();
    let is_loading = ctrl.records.is_loading();
    let active_source = selected_filter
        .and_then(|idx| filters.get(idx as usize).map(|f| f.source));

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }

        div { class: "analyze-arena analyze-arena--records",
            div { class: "arena",
                // ── Topbar ───────────────────────────────────
                header { class: "arena-topbar", role: "banner",
                    button {
                        r#type: "button",
                        class: "back-btn",
                        "aria-label": "{tr.detail_back_btn_aria}",
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
                        class: "arena-topbar__logo",
                        alt: "Space logo",
                        src: "{space_logo}",
                    }
                    nav { class: "breadcrumb",
                        span { class: "breadcrumb__item", "{space_title}" }
                        span { class: "breadcrumb__sep", "›" }
                        span { class: "breadcrumb__item", "{report_name}" }
                        span { class: "breadcrumb__sep", "›" }
                        span { class: "breadcrumb__item breadcrumb__current", "{tr.records_page_title}" }
                    }
                    span { class: "topbar-title", "{tr.records_page_title}" }
                }

                // ── Body ─────────────────────────────────────
                section { class: "records-body",
                    header { class: "records-head",
                        h1 { "{tr.records_page_title}" }
                        p { "{tr.records_page_hint}" }
                    }

                    // ── Filter chip strip ───────────────────
                    div { class: "records-chips", role: "tablist",
                        for (idx, f) in filters.iter().enumerate() {
                            {
                                let src = f.source.as_str();
                                let badge = f.source_label.clone();
                                let label = f.label.clone();
                                let is_active = selected_filter == Some(idx as u32);
                                rsx! {
                                    button {
                                        key: "records-chip-{idx}",
                                        r#type: "button",
                                        class: "records-chip",
                                        role: "tab",
                                        "aria-pressed": is_active,
                                        "data-source": "{src}",
                                        onclick: move |_| {
                                            ctrl.selected_filter.set(Some(idx as u32));
                                            ctrl.records.refresh();
                                        },
                                        span { class: "records-chip__source", "{badge}" }
                                        span { "{label}" }
                                    }
                                }
                            }
                        }
                    }

                    // ── Records table ───────────────────────
                    if let Some(source) = active_source {
                        RecordsTable { source, rows: records.clone() }
                        if records.is_empty() && !is_loading {
                            div { class: "records-empty", "{tr.records_empty}" }
                        }
                        if has_more {
                            div { class: "records-loadmore",
                                button {
                                    r#type: "button",
                                    class: "records-loadmore-btn",
                                    disabled: is_loading,
                                    onclick: move |_| ctrl.records.next(),
                                    "{tr.records_load_more}"
                                }
                            }
                        }
                    } else {
                        div { class: "records-empty", "{tr.records_page_hint}" }
                    }
                }
            }
        }
    }
}

/// Per-source table. Discussion/follow fields stay empty for poll/quiz
/// sources, so we render columns conditional on `source` rather than
/// fanning out per-source components — keeps the row markup uniform.
/// Public so the CREATE wizard's preview card can reuse it. Loads
/// its own un-host-scoped stylesheet so it works inside any
/// `.analyze-arena--*` host without per-host duplication.
///
/// Header and body are split into two separate `<table>`s inside the
/// wrap so the records page can pin a non-scrolling header strip
/// above an internally-scrolling body — the scrollbar then sits next
/// to the rows only and never crosses the header. Both tables share
/// the same `data-source` attribute on the wrap so the source-keyed
/// column-width rules in `table.css` apply identically to header and
/// body cells, keeping columns aligned even though the markup is
/// split. `table-layout: fixed` (declared in records/style.css)
/// makes the explicit widths authoritative.
#[component]
pub fn RecordsTable(source: AnalyzeFilterSource, rows: Vec<AnalyzeRecordRow>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    if rows.is_empty() {
        return rsx! {};
    }

    let src = source.as_str();
    rsx! {
        document::Stylesheet { href: asset!("./table.css") }
        div { class: "records-table-wrap", "data-source": "{src}",
            div { class: "records-table-header",
                table { class: "records-table",
                    thead {
                        tr {
                            th { "{tr.records_col_user}" }
                            match source {
                                AnalyzeFilterSource::Poll | AnalyzeFilterSource::Quiz => rsx! {
                                    th { "{tr.records_col_question}" }
                                    th { "{tr.records_col_answer}" }
                                },
                                AnalyzeFilterSource::Discussion => rsx! {
                                    th { "{tr.records_col_post_title}" }
                                    th { "{tr.records_col_comment}" }
                                },
                                AnalyzeFilterSource::Follow => rsx! {
                                    th { "{tr.records_col_target}" }
                                },
                            }
                        }
                    }
                }
            }
            div { class: "records-table-body",
                table { class: "records-table",
                    tbody {
                        for (idx, r) in rows.iter().enumerate() {
                            {
                                let row = r.clone();
                                rsx! {
                                    tr { key: "rec-{idx}",
                                        td {
                                            UserCell {
                                                display_name: row.user_display_name.clone(),
                                                username: row.user_username.clone(),
                                                profile_url: row.user_profile_url.clone(),
                                                fallback_pk: row.user_pk.clone(),
                                            }
                                        }
                                        match source {
                                            AnalyzeFilterSource::Poll | AnalyzeFilterSource::Quiz => rsx! {
                                                td { "{row.question_text}" }
                                                td { "{row.answer_text}" }
                                            },
                                            AnalyzeFilterSource::Discussion => rsx! {
                                                td { "{row.post_title}" }
                                                td { "{row.comment_text}" }
                                            },
                                            AnalyzeFilterSource::Follow => rsx! {
                                                td {
                                                    UserCell {
                                                        display_name: row.target_display_name.clone(),
                                                        username: row.target_username.clone(),
                                                        profile_url: String::new(),
                                                        fallback_pk: row.target_pk.clone(),
                                                    }
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
        }
    }
}

#[component]
pub fn UserCell(
    display_name: String,
    username: String,
    profile_url: String,
    fallback_pk: String,
) -> Element {
    let name = if !display_name.is_empty() {
        display_name.clone()
    } else if !username.is_empty() {
        username.clone()
    } else {
        // Last-resort: show a truncated pk so the row never reads as
        // "anonymous" — the user has at least an opaque handle.
        let trimmed = fallback_pk.split('#').last().unwrap_or(&fallback_pk);
        let len = trimmed.len();
        if len > 8 {
            format!("{}…", &trimmed[..8])
        } else {
            trimmed.to_string()
        }
    };
    let handle = if !username.is_empty() {
        format!("@{username}")
    } else {
        String::new()
    };
    let initial = name.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default();

    rsx! {
        div { class: "records-cell-user",
            if !profile_url.is_empty() {
                img { class: "records-avatar", src: "{profile_url}", alt: "" }
            } else {
                span { class: "records-avatar-fallback", "{initial}" }
            }
            span { class: "records-user-meta",
                span { class: "records-user-name", "{name}" }
                if !handle.is_empty() {
                    span { class: "records-user-handle", "{handle}" }
                }
            }
        }
    }
}
