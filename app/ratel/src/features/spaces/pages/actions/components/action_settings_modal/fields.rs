use crate::features::spaces::pages::actions::*;

const TIME_ZONE_OPTIONS: [&str; 3] = ["UTC", "Asia/Seoul", "America/New_York"];

#[component]
pub fn DateField(
    label: String,
    value: String,
    placeholder: String,
    min: Option<String>,
    onchange: EventHandler<String>,
) -> Element {
    let display_value = if value.is_empty() {
        placeholder
    } else {
        format_date(&value)
    };
    let input_text_class = if value.is_empty() {
        "w-full pr-8 font-inter text-[12px]/[16px] font-semibold text-web-font-neutral outline-none disabled:pointer-events-none disabled:opacity-100"
    } else {
        "w-full pr-8 font-inter text-[12px]/[16px] font-semibold text-web-font-primary outline-none disabled:pointer-events-none disabled:opacity-100"
    };

    rsx! {
        div { class: "flex w-full flex-col gap-1.5 min-w-0",
            p { class: "font-semibold font-raleway text-[12px]/[16px] text-web-font-neutral",
                {label}
            }

            label { class: "relative flex h-8 w-full min-w-0 items-center justify-between rounded-[8px] border-[0.5px] border-gray-600 bg-web-input px-3",
                Input {
                    variant: InputVariant::Plain,
                    class: input_text_class.to_string(),
                    value: display_value,
                    disabled: true,
                }

                icons::calendar::CalendarToday {
                    width: "20",
                    height: "20",
                    class: "shrink-0 text-web-font-primary [&>path]:stroke-current",
                }

                input {
                    class: "absolute inset-0 cursor-pointer opacity-0",
                    r#type: "date",
                    value,
                    min: min.unwrap_or_default(),
                    onchange: move |e| onchange.call(e.value()),
                }
            }
        }
    }
}

#[component]
pub fn TimeZoneField(
    value: String,
    placeholder: String,
    onchange: EventHandler<String>,
) -> Element {
    let display_value = if value.is_empty() {
        placeholder.clone()
    } else {
        value.clone()
    };
    let input_text_class = if value.is_empty() {
        "w-full pr-8 font-inter text-[12px]/[16px] font-semibold text-web-font-neutral outline-none disabled:pointer-events-none disabled:opacity-100"
    } else {
        "w-full pr-8 font-inter text-[12px]/[16px] font-semibold text-web-font-primary outline-none disabled:pointer-events-none disabled:opacity-100"
    };

    rsx! {
        div { class: "flex flex-col gap-1.5",
            p { class: "font-semibold font-raleway text-[12px]/[16px] text-web-font-neutral",
                {placeholder.clone()}
            }

            label { class: "relative flex h-11 w-full items-center justify-between rounded-[8px] border-[0.5px] border-gray-600 bg-web-input px-3",
                Input {
                    variant: InputVariant::Plain,
                    class: input_text_class.to_string(),
                    value: display_value,
                    disabled: true,
                }

                icons::internet_script::Internet {
                    width: "20",
                    height: "20",
                    class: "shrink-0 text-web-font-primary [&>path]:stroke-current [&>path]:fill-current [&>circle]:stroke-current",
                }

                select {
                    class: "absolute inset-0 cursor-pointer opacity-0",
                    value: value.clone(),
                    onchange: move |e| onchange.call(e.value()),
                    option { value: "", disabled: true, {placeholder.clone()} }
                    for option in TIME_ZONE_OPTIONS {
                        option { value: option, {option} }
                    }
                }
            }
        }
    }
}

fn format_date(value: &str) -> String {
    let mut parts = value.split('-');
    match (parts.next(), parts.next(), parts.next()) {
        (Some(year), Some(month), Some(day)) => format!("{month}-{day}-{year}"),
        _ => value.to_string(),
    }
}
