use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::features::spaces::pages::actions::actions::meet::*;
use crate::*;

fn format_datetime_local(ts_ms: i64) -> String {
    use chrono::{TimeZone, Utc};
    Utc.timestamp_millis_opt(ts_ms)
        .single()
        .map(|dt| dt.format("%Y-%m-%dT%H:%M").to_string())
        .unwrap_or_default()
}

fn parse_datetime_local(s: &str) -> Option<i64> {
    use chrono::{NaiveDateTime, TimeZone, Utc};
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
        .ok()
        .and_then(|ndt| Utc.from_local_datetime(&ndt).single())
        .map(|dt| dt.timestamp_millis())
}

#[component]
pub fn MeetWhenCard() -> Element {
    let tr: MeetActionTranslate = use_translate();
    let UseMeet {
        meet,
        mut update_start_time,
        mut update_duration,
        ..
    } = use_context::<UseMeet>();
    let current = meet();
    let mode = current.mode.clone();
    let start_value = format_datetime_local(current.start_time);
    let duration = current.duration_min;

    let start_disabled = mode == MeetMode::Instant;

    let on_start_change = move |e: FormEvent| {
        if let Some(ts) = parse_datetime_local(&e.value()) {
            update_start_time.call(ts);
        }
    };
    let dec = move |_| {
        let next = (duration - 15).max(15);
        update_duration.call(next);
    };
    let inc = move |_| {
        let next = (duration + 15).min(1440);
        update_duration.call(next);
    };

    rsx! {
        section { class: "meet-card",
            header { class: "meet-card__head",
                h2 { class: "meet-card__title meet-card__title--meet", "{tr.when_label}" }
            }
            div { class: "when-row",
                div { class: "field",
                    label { class: "field__label", "{tr.when_start_label}" }
                    input {
                        class: "field__input",
                        r#type: "datetime-local",
                        "data-testid": "meet-start-time",
                        disabled: start_disabled,
                        value: "{start_value}",
                        onchange: on_start_change,
                    }
                }
                div { class: "field",
                    label { class: "field__label", "{tr.when_duration_label}" }
                    div { class: "dur",
                        button {
                            class: "dur__step",
                            "data-testid": "meet-duration-dec",
                            onclick: dec,
                            "−"
                        }
                        span {
                            class: "dur__value",
                            "data-testid": "meet-duration-value",
                            "{duration}"
                            small { " {tr.when_duration_unit_min}" }
                        }
                        button {
                            class: "dur__step",
                            "data-testid": "meet-duration-inc",
                            onclick: inc,
                            "+"
                        }
                    }
                }
            }
        }
    }
}
