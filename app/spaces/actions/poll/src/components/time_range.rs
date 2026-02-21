use crate::*;

#[component]
pub fn TimeRangeDisplay(started_at: i64, ended_at: i64) -> Element {
    let start_str = format_timestamp(started_at);
    let end_str = format_timestamp(ended_at);

    rsx! {
        div { class: "flex items-center gap-2 text-sm text-neutral-400",
            span { "{start_str}" }
            span { "~" }
            span { "{end_str}" }
        }
    }
}

#[component]
pub fn TimeRangeSetting(
    started_at: i64,
    ended_at: i64,
    on_change: EventHandler<(i64, i64)>,
) -> Element {
    let start_str = format_timestamp(started_at);
    let end_str = format_timestamp(ended_at);

    rsx! {
        div { class: "flex flex-col gap-2",
            div { class: "flex items-center gap-2 text-sm text-neutral-300",
                span { "Start: {start_str}" }
                span { "~" }
                span { "End: {end_str}" }
            }
        }
    }
}

fn format_timestamp(ts: i64) -> String {
    let secs = ts / 1000;
    let naive = chrono::DateTime::from_timestamp(secs, 0);
    match naive {
        Some(dt) => dt.format("%Y-%m-%d %H:%M").to_string(),
        None => "Invalid date".to_string(),
    }
}
