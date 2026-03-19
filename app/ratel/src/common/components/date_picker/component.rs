use dioxus::prelude::*;

use dioxus_primitives::{
    date_picker::{
        self, DatePickerInputProps as OriginDatePickerInputProps, DatePickerProps,
        DateRangePickerProps,
    },
    popover::{PopoverContentProps, PopoverTriggerProps},
    ContentAlign,
};
use time::{ext::NumericalDuration, format_description, Date, OffsetDateTime};

use strum::IntoEnumIterator;

use crate::common::components::time_picker::TimePicker;

use super::super::calendar::*;
use super::super::popover::*;
use super::super::select::*;

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumIter, strum::EnumCount)]
pub enum Timezone {
    Pacific,
    Mountain,
    Central,
    Eastern,
    Utc,
    London,
    Paris,
    Istanbul,
    Dubai,
    Kolkata,
    Bangkok,
    Shanghai,
    Seoul,
    Sydney,
}

impl Timezone {
    pub const fn label(&self) -> &'static str {
        match self {
            Timezone::Pacific => "Pacific (UTC-8)",
            Timezone::Mountain => "Mountain (UTC-7)",
            Timezone::Central => "Central (UTC-6)",
            Timezone::Eastern => "Eastern (UTC-5)",
            Timezone::Utc => "UTC (UTC+0)",
            Timezone::London => "London (UTC+0)",
            Timezone::Paris => "Paris (UTC+1)",
            Timezone::Istanbul => "Istanbul (UTC+3)",
            Timezone::Dubai => "Dubai (UTC+4)",
            Timezone::Kolkata => "Kolkata (UTC+5:30)",
            Timezone::Bangkok => "Bangkok (UTC+7)",
            Timezone::Shanghai => "Shanghai (UTC+8)",
            Timezone::Seoul => "Seoul (UTC+9)",
            Timezone::Sydney => "Sydney (UTC+11)",
        }
    }

    pub const fn short_label(&self) -> &'static str {
        match self {
            Timezone::Pacific => "PT",
            Timezone::Mountain => "MT",
            Timezone::Central => "CT",
            Timezone::Eastern => "ET",
            Timezone::Utc => "UTC",
            Timezone::London => "GMT",
            Timezone::Paris => "CET",
            Timezone::Istanbul => "TRT",
            Timezone::Dubai => "GST",
            Timezone::Kolkata => "IST",
            Timezone::Bangkok => "ICT",
            Timezone::Shanghai => "CST",
            Timezone::Seoul => "KST",
            Timezone::Sydney => "AEDT",
        }
    }
}

impl Timezone {
    /// Maps an IANA timezone name (e.g. "America/New_York") to the closest
    /// `Timezone` variant. Returns `None` for unrecognized names.
    pub fn from_iana(iana: &str) -> Option<Self> {
        match iana {
            // Pacific (UTC-8)
            "America/Los_Angeles"
            | "America/Vancouver"
            | "America/Tijuana"
            | "US/Pacific"
            | "PST8PDT" => Some(Timezone::Pacific),

            // Mountain (UTC-7)
            "America/Denver" | "America/Edmonton" | "America/Phoenix" | "US/Mountain"
            | "MST7MDT" => Some(Timezone::Mountain),

            // Central (UTC-6)
            "America/Chicago"
            | "America/Winnipeg"
            | "America/Mexico_City"
            | "US/Central"
            | "CST6CDT" => Some(Timezone::Central),

            // Eastern (UTC-5)
            "America/New_York" | "America/Toronto" | "America/Detroit" | "US/Eastern"
            | "EST5EDT" => Some(Timezone::Eastern),

            // UTC (UTC+0)
            "UTC" | "Etc/UTC" | "Etc/GMT" | "GMT" => Some(Timezone::Utc),

            // London (UTC+0 / UTC+1 BST)
            "Europe/London" | "Europe/Dublin" | "Europe/Lisbon" => Some(Timezone::London),

            // Paris (UTC+1)
            "Europe/Paris" | "Europe/Berlin" | "Europe/Rome" | "Europe/Madrid"
            | "Europe/Amsterdam" | "Europe/Brussels" | "Europe/Vienna" | "Europe/Zurich"
            | "Europe/Stockholm" | "Europe/Oslo" | "Europe/Copenhagen" | "Europe/Warsaw"
            | "Europe/Prague" | "Europe/Budapest" | "CET" => Some(Timezone::Paris),

            // Istanbul (UTC+3)
            "Europe/Istanbul" | "Europe/Moscow" | "Europe/Minsk" | "Asia/Riyadh"
            | "Asia/Baghdad" | "Africa/Nairobi" => Some(Timezone::Istanbul),

            // Dubai (UTC+4)
            "Asia/Dubai" | "Asia/Muscat" | "Asia/Tbilisi" | "Asia/Baku" => Some(Timezone::Dubai),

            // Kolkata (UTC+5:30)
            "Asia/Kolkata" | "Asia/Calcutta" | "Asia/Colombo" => Some(Timezone::Kolkata),

            // Bangkok (UTC+7)
            "Asia/Bangkok" | "Asia/Jakarta" | "Asia/Ho_Chi_Minh" | "Asia/Saigon" => {
                Some(Timezone::Bangkok)
            }

            // Shanghai (UTC+8)
            "Asia/Shanghai" | "Asia/Hong_Kong" | "Asia/Taipei" | "Asia/Singapore"
            | "Asia/Kuala_Lumpur" | "Asia/Makassar" | "Asia/Manila" | "PRC" => {
                Some(Timezone::Shanghai)
            }

            // Seoul (UTC+9)
            "Asia/Seoul" | "Asia/Tokyo" | "Japan" | "ROK" => Some(Timezone::Seoul),

            // Sydney (UTC+10/+11)
            "Australia/Sydney"
            | "Australia/Melbourne"
            | "Australia/Brisbane"
            | "Australia/Hobart"
            | "Pacific/Auckland"
            | "Pacific/Fiji" => Some(Timezone::Sydney),

            _ => None,
        }
    }

    /// Detects the user's local timezone from the browser environment.
    /// Returns `Utc` on the server side or if detection fails.
    pub fn detect_local() -> Self {
        #[cfg(not(feature = "server"))]
        {
            js_sys::eval("Intl.DateTimeFormat().resolvedOptions().timeZone")
                .ok()
                .and_then(|v| v.as_string())
                .and_then(|iana| Self::from_iana(&iana))
                .unwrap_or(Timezone::Utc)
        }
        #[cfg(feature = "server")]
        {
            Timezone::Utc
        }
    }
}

impl std::fmt::Display for Timezone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.short_label())
    }
}

#[component]
pub fn TimezonePicker(#[props(default)] on_change: EventHandler<Timezone>) -> Element {
    let local_tz = Timezone::detect_local();

    let options = Timezone::iter().enumerate().map(|(i, tz)| {
        rsx! {
            SelectOption::<Option<Timezone>> { index: i, value: tz, text_value: "{tz}",
                "{tz.label()}"
                SelectItemIndicator {}
            }
        }
    });

    rsx! {
        div { class: "timezone-picker @max-sm:w-full",
            Select::<Option<Timezone>> {
                class: "@max-sm:w-full",
                placeholder: "Timezone",
                default_value: Some(local_tz),
                on_value_change: move |v: Option<Option<Timezone>>| {
                    if let Some(Some(tz)) = v {
                        on_change.call(tz);
                    }
                },
                SelectTrigger {
                    class: "@max-sm:w-full",
                    aria_label: "Select Timezone",
                    min_width: "7rem",
                    SelectValue {}
                }
                SelectList { aria_label: "Timezone",
                    SelectGroup { {options} }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateTimeRange {
    pub start_date: Option<Date>,
    pub start_hour: u8,
    pub start_minute: u8,
    pub end_date: Option<Date>,
    pub end_hour: u8,
    pub end_minute: u8,
}

/// Converts a date + hour + minute into total minutes since the Unix epoch (UTC),
/// used to compute and preserve the duration between start and end.
fn to_total_minutes(date: Date, hour: u8, minute: u8) -> i64 {
    let datetime = date.with_hms(hour, minute, 0).expect("valid time");
    let offset_datetime = datetime.assume_utc();
    offset_datetime.unix_timestamp() / 60
}

/// Converts total minutes (since Unix epoch) back into (Date, hour, minute).
fn from_total_minutes(total: i64) -> (Date, u8, u8) {
    let offset_datetime =
        OffsetDateTime::from_unix_timestamp(total * 60).expect("valid timestamp");
    (
        offset_datetime.date(),
        offset_datetime.hour(),
        offset_datetime.minute(),
    )
}

#[component]
pub fn DateAndTimePicker(#[props(default)] on_change: EventHandler<DateTimeRange>) -> Element {
    let now = OffsetDateTime::now_utc();
    let next_hour = (now.hour() + 1) % 24;
    let today = now.date();
    let tomorrow = today.saturating_add(1.days());
    let mut selected_start_date = use_signal(|| Some(today));
    let mut selected_end_date = use_signal(|| Some(tomorrow));
    let mut start_hour = use_signal(move || next_hour);
    let mut start_minute = use_signal(|| 0u8);
    let mut end_hour = use_signal(move || next_hour);
    let mut end_minute = use_signal(|| 0u8);
    let format = format_description::parse("[year]-[month]-[day]").unwrap();

    let emit = move || {
        on_change.call(DateTimeRange {
            start_date: selected_start_date(),
            start_hour: start_hour(),
            start_minute: start_minute(),
            end_date: selected_end_date(),
            end_hour: end_hour(),
            end_minute: end_minute(),
        });
    };

    // Computes the duration (in minutes) between the current start and end.
    let get_duration = move || -> i64 {
        let sd = selected_start_date().unwrap_or(today);
        let ed = selected_end_date().unwrap_or(tomorrow);
        let start_total = to_total_minutes(sd, start_hour(), start_minute());
        let end_total = to_total_minutes(ed, end_hour(), end_minute());
        (end_total - start_total).max(1)
    };

    // Adjusts the end date/time to preserve the given duration from the current start.
    let mut adjust_end = move |duration_minutes: i64| {
        let sd = selected_start_date().unwrap_or(today);
        let new_end_total =
            to_total_minutes(sd, start_hour(), start_minute()) + duration_minutes;
        let (new_end_date, new_end_h, new_end_m) = from_total_minutes(new_end_total);
        selected_end_date.set(Some(new_end_date));
        end_hour.set(new_end_h);
        end_minute.set(new_end_m);
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "flex items-center w-full @container",
            div { class: "flex flex-row gap-4 items-center w-full @max-sm:flex-col",
                div { class: "flex flex-row flex-1 gap-4 items-center min-w-0 @max-sm:flex-col @max-sm:w-full",
                    DatePicker {
                        selected_date: selected_start_date(),
                        on_value_change: move |v| {
                            let duration = get_duration();
                            selected_start_date.set(v);
                            adjust_end(duration);
                            emit();
                        },
                        DatePickerInput { date: selected_start_date().and_then(|d| d.format(&format).ok()).unwrap_or_default() }
                    }
                    TimePicker {
                        hour: start_hour(),
                        minute: start_minute(),
                        on_change: move |(h, m)| {
                            let duration = get_duration();
                            start_hour.set(h);
                            start_minute.set(m);
                            adjust_end(duration);
                            emit();
                        },
                    }
                }

                div { class: "h-[0.5px] w-[15px] bg-text-secondary max-tablet:hidden" }

                div { class: "flex flex-row flex-1 gap-4 items-center min-w-0 @max-sm:flex-col @max-sm:w-full",
                    DatePicker {
                        selected_date: selected_end_date(),
                        on_value_change: move |v| {
                            selected_end_date.set(v);
                            emit();
                        },
                        DatePickerInput { date: selected_end_date().and_then(|d| d.format(&format).ok()).unwrap_or_default() }
                    }
                    TimePicker {
                        hour: end_hour(),
                        minute: end_minute(),
                        on_change: move |(h, m)| {
                            end_hour.set(h);
                            end_minute.set(m);
                            emit();
                        },
                    }
                }

                TimezonePicker {}
            }
        }
    }
}

#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    rsx! {
        div { class: "flex flex-1 min-w-0 @max-sm:w-full",
            date_picker::DatePicker {
                class: "flex-1 w-full date-picker",
                on_value_change: props.on_value_change,
                selected_date: props.selected_date,
                disabled: props.disabled,
                read_only: props.read_only,
                min_date: props.min_date,
                max_date: props.max_date,
                month_count: props.month_count,
                disabled_ranges: props.disabled_ranges,
                roving_loop: props.roving_loop,
                attributes: props.attributes,
                date_picker::DatePickerPopover {
                    class: "flex flex-1 w-full grow",
                    popover_root: PopoverRoot,
                    {props.children}
                }
            }
        }
    }
}

#[component]
pub fn DateRangePicker(props: DateRangePickerProps) -> Element {
    rsx! {
        div {
            date_picker::DateRangePicker {
                class: "date-picker",
                on_range_change: props.on_range_change,
                selected_range: props.selected_range,
                disabled: props.disabled,
                read_only: props.read_only,
                min_date: props.min_date,
                max_date: props.max_date,
                month_count: props.month_count,
                disabled_ranges: props.disabled_ranges,
                roving_loop: props.roving_loop,
                attributes: props.attributes,
                date_picker::DatePickerPopover { popover_root: PopoverRoot, {props.children} }
            }
        }
    }
}

#[component]
pub fn DatePickerInput(#[props(default)] date: String) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "flex-1 w-full grow date-picker-group @max-sm:w-full",
            DatePickerPopoverTrigger {
                div { class: "flex flex-row justify-between items-center w-full min-w-0 h-8 rounded-[8px] @max-sm:w-full",
                    span { class: "grow", {date} }

                    icons::calendar::CalendarToday {
                        width: "20",
                        height: "20",
                        class: "border-[0px] shrink-0 text-web-font-primary [&>path]:stroke-current",
                    }
                }
            }
            DatePickerPopoverContent { align: ContentAlign::Center,
                date_picker::DatePickerCalendar { calendar: Calendar,
                    CalendarView {
                        CalendarHeader {
                            CalendarNavigation {
                                CalendarPreviousMonthButton {}
                                CalendarSelectMonth {}
                                CalendarSelectYear {}
                                CalendarNextMonthButton {}
                            }
                        }
                        CalendarGrid {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn DateRangePickerInput(props: OriginDatePickerInputProps) -> Element {
    rsx! {
        date_picker::DateRangePickerInput {
            on_format_day_placeholder: props.on_format_day_placeholder,
            on_format_month_placeholder: props.on_format_month_placeholder,
            on_format_year_placeholder: props.on_format_year_placeholder,
            attributes: props.attributes,
            {props.children}
            DatePickerPopoverTrigger {}
            DatePickerPopoverContent { align: ContentAlign::Center,
                date_picker::DateRangePickerCalendar { calendar: RangeCalendar,
                    CalendarView {
                        CalendarHeader {
                            CalendarNavigation {
                                CalendarPreviousMonthButton {}
                                CalendarSelectMonth {}
                                CalendarSelectYear {}
                                CalendarNextMonthButton {}
                            }
                        }
                        CalendarGrid {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn DatePickerPopoverTrigger(props: PopoverTriggerProps) -> Element {
    rsx! {
        PopoverTrigger { aria_label: "Show Calendar", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn DatePickerPopoverContent(props: PopoverContentProps) -> Element {
    rsx! {
        PopoverContent {
            class: "popover-content",
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}
