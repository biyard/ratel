use crate::features::spaces::pages::report::types::*;
use crate::features::spaces::pages::report::*;
use crate::*;

/// 4-level slash autocomplete popup — `/data` → `:analyze` → `:source`
/// → `:item`. Pure consumer: option list and apply logic both live on
/// `UseReportDetailContext` so the keyboard handler in `DocBlock`
/// stays in sync with mouse clicks here.
#[component]
pub fn SlashPopup() -> Element {
    let mut ctx = use_report_detail_context();
    let state = ctx.slash.read().clone();
    let Some(state) = state else {
        return rsx! {};
    };

    let heading = match state.level {
        0 => "COMMAND",
        1 => "ANALYZE",
        2 => "DATA SOURCE",
        _ => "ITEMS",
    };
    // Coords come from the editor's slash watcher and are viewport-
    // relative (from `getBoundingClientRect`). The popup is positioned
    // `fixed` so we don't have to compute scroll offsets. For
    // `placement = "above"` we translate the popup up by its own height
    // via `transform: translateY(-100%)` so JS doesn't need to know the
    // rendered popup height ahead of time.
    let style = if state.placement == "above" {
        format!(
            "position: fixed; top: {top}px; left: {left}px; transform: translateY(-100%);",
            top = state.caret_y,
            left = state.caret_x
        )
    } else {
        format!(
            "position: fixed; top: {top}px; left: {left}px;",
            top = state.caret_y,
            left = state.caret_x
        )
    };

    let options = ctx.slash_options();
    let selected = state.selected_index;

    rsx! {
        div {
            class: "report-detail__slash-pop",
            "data-placement": "{state.placement}",
            role: "listbox",
            style: "{style}",
            div { class: "report-detail__slash-pop-head",
                span { class: "report-detail__slash-pop-heading", "{heading}" }
                span { class: "report-detail__slash-pop-hint", "↑↓ 이동 · ↵ 선택 · esc 닫기" }
            }
            div { class: "report-detail__slash-pop-list",
                if options.is_empty() {
                    div { class: "report-detail__slash-pop-empty", "일치하는 항목 없음" }
                } else {
                    for (i, opt) in options.into_iter().enumerate() {
                        SlashItem {
                            key: "{opt.id}",
                            opt,
                            selected: i == selected,
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SlashItem(opt: SlashOption, selected: bool) -> Element {
    let mut ctx = use_report_detail_context();
    let action = opt.action.clone();
    rsx! {
        button {
            class: "report-detail__slash-item",
            "aria-selected": selected,
            r#type: "button",
            // `mousedown` (not click) so the focused contenteditable
            // doesn't lose its selection before our handler runs.
            onmousedown: move |e| {
                e.prevent_default();
                ctx.apply_slash_action(&action);
            },
            span { class: "report-detail__slash-item-title", "{opt.title}" }
            span { class: "report-detail__slash-item-meta", "{opt.meta}" }
            if !opt.filters.is_empty() {
                div { class: "report-detail__xf-chips report-detail__xf-chips--slash",
                    for chip in opt.filters.iter() {
                        span {
                            class: "report-detail__xf-chip",
                            "data-source": chip.source.as_token(),
                            span { class: "report-detail__xf-chip-src", "{slash_src_label(chip.source)}" }
                            span { class: "report-detail__xf-chip-label", "{chip.label}" }
                        }
                    }
                }
            }
        }
    }
}

fn slash_src_label(s: ActionSource) -> &'static str {
    match s {
        ActionSource::Poll => "POLL",
        ActionSource::Quiz => "QUIZ",
        ActionSource::Discussion => "DISCUSSION",
        ActionSource::Follow => "FOLLOW",
    }
}
