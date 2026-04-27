use dioxus::prelude::*;

use dioxus_primitives::{
    date_picker::{
        self, DatePickerInputProps as OriginDatePickerInputProps, DatePickerProps,
        DateRangePickerProps,
    },
    popover::{PopoverContentProps, PopoverTriggerProps},
    ContentAlign,
};
use time::{ext::NumericalDuration, format_description, Date, OffsetDateTime, UtcOffset};

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

    /// Fixed offset from UTC in whole seconds. Handles half-hour offsets
    /// like Kolkata (+5:30). Note: this does not account for daylight
    /// saving time — we intentionally use the standard (non-DST) offset
    /// so the stored timestamp is deterministic from the picker input.
    pub const fn offset_seconds(&self) -> i32 {
        match self {
            Timezone::Pacific => -8 * 3600,
            Timezone::Mountain => -7 * 3600,
            Timezone::Central => -6 * 3600,
            Timezone::Eastern => -5 * 3600,
            Timezone::Utc => 0,
            Timezone::London => 0,
            Timezone::Paris => 3600,
            Timezone::Istanbul => 3 * 3600,
            Timezone::Dubai => 4 * 3600,
            Timezone::Kolkata => 5 * 3600 + 30 * 60,
            Timezone::Bangkok => 7 * 3600,
            Timezone::Shanghai => 8 * 3600,
            Timezone::Seoul => 9 * 3600,
            Timezone::Sydney => 11 * 3600,
        }
    }

    /// Convert a wall-clock (date + hour + minute) in this timezone to a
    /// UTC unix timestamp in milliseconds.
    pub fn local_to_utc_millis(&self, date: Date, hour: u8, minute: u8) -> i64 {
        let offset = UtcOffset::from_whole_seconds(self.offset_seconds())
            .unwrap_or(UtcOffset::UTC);
        let datetime = date.with_hms(hour, minute, 0).expect("valid time");
        datetime.assume_offset(offset).unix_timestamp() * 1000
    }

    /// Convert a UTC unix timestamp (milliseconds) to wall-clock
    /// (date + hour + minute) in this timezone.
    pub fn utc_millis_to_local(&self, ms: i64) -> (Date, u8, u8) {
        let offset = UtcOffset::from_whole_seconds(self.offset_seconds())
            .unwrap_or(UtcOffset::UTC);
        let dt = OffsetDateTime::from_unix_timestamp(ms / 1000)
            .unwrap_or(OffsetDateTime::UNIX_EPOCH)
            .to_offset(offset);
        (dt.date(), dt.hour(), dt.minute())
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
pub fn TimezonePicker(
    #[props(default)] value: Option<Timezone>,
    #[props(default)] on_change: EventHandler<Timezone>,
) -> Element {
    let initial_tz = value.unwrap_or_else(Timezone::detect_local);

    let options = Timezone::iter().enumerate().map(|(i, tz)| {
        rsx! {
            SelectOption::<Option<Timezone>> { index: i, value: tz, text_value: "{tz}",
                "{tz.label()}"
                SelectItemIndicator {}
            }
        }
    });

    rsx! {
        div { class: "timezone-picker @max-mobile:w-full",
            Select::<Option<Timezone>> {
                class: "@max-mobile:w-full",
                placeholder: "Timezone",
                default_value: Some(initial_tz),
                on_value_change: move |v: Option<Option<Timezone>>| {
                    if let Some(Some(tz)) = v {
                        on_change.call(tz);
                    }
                },
                SelectTrigger {
                    class: "@max-mobile:w-full",
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
    /// Timezone the user selected in the picker. Callers should use this
    /// to convert the wall-clock `(date, hour, minute)` values back to
    /// UTC millis via `Timezone::local_to_utc_millis`.
    pub timezone: Timezone,
}

#[component]
pub fn DateAndTimePicker(
    #[props(default)] on_change: EventHandler<DateTimeRange>,
    #[props(default)] initial_started_at: Option<i64>,
    #[props(default)] initial_ended_at: Option<i64>,
) -> Element {
    const ONE_DAY_MS: i64 = 86_400_000;
    const ONE_HOUR_MS: i64 = 3_600_000;
    const ONE_MIN_MS: i64 = 60_000;

    let now_ms = OffsetDateTime::now_utc().unix_timestamp() * 1000;
    // Default: next full hour from now, duration 1 day. Snap to the hour so
    // the displayed default matches the TimePicker defaults.
    let default_start_ms = {
        let hour = ONE_HOUR_MS;
        ((now_ms + hour) / hour) * hour
    };

    let (init_start_ms, init_end_ms) = match (initial_started_at, initial_ended_at) {
        (Some(s), Some(e)) if e >= s => (s, e),
        (Some(s), Some(_)) => (s, s + ONE_DAY_MS),
        (Some(s), None) => (s, s + ONE_DAY_MS),
        (None, Some(e)) => (e - ONE_DAY_MS, e),
        (None, None) => (default_start_ms, default_start_ms + ONE_DAY_MS),
    };

    // Source of truth: raw UTC millis. Wall-clock display is derived from
    // `(raw_ms, tz)`, so switching timezone re-renders automatically and
    // the round-trip stays consistent.
    let mut raw_start = use_signal(move || init_start_ms);
    let mut raw_end = use_signal(move || init_end_ms);
    let mut tz = use_signal(Timezone::detect_local);

    let display_start = use_memo(move || tz().utc_millis_to_local(raw_start()));
    let display_end = use_memo(move || tz().utc_millis_to_local(raw_end()));

    let format = format_description::parse("[year]-[month]-[day]").unwrap();

    let emit = move || {
        let (sd, sh, sm) = display_start();
        let (ed, eh, em) = display_end();
        on_change.call(DateTimeRange {
            start_date: Some(sd),
            start_hour: sh,
            start_minute: sm,
            end_date: Some(ed),
            end_hour: eh,
            end_minute: em,
            timezone: tz(),
        });
    };

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        div { class: "flex items-center w-full @container",
            div { class: "flex flex-row gap-4 items-center w-full @max-mobile:flex-col",
                div { class: "flex flex-row flex-1 gap-4 items-center min-w-0 @max-mobile:flex-col @max-mobile:w-full",
                    DatePicker {
                        selected_date: Some(display_start().0),
                        on_value_change: move |v: Option<Date>| {
                            if let Some(new_date) = v {
                                let duration = (raw_end() - raw_start()).max(ONE_MIN_MS);
                                let (_, h, m) = display_start();
                                let new_start = tz().local_to_utc_millis(new_date, h, m);
                                raw_start.set(new_start);
                                raw_end.set(new_start + duration);
                                emit();
                            }
                        },
                        DatePickerInput { date: display_start().0.format(&format).unwrap_or_default() }
                    }
                    TimePicker {
                        hour: display_start().1,
                        minute: display_start().2,
                        on_change: move |(h, m)| {
                            let duration = (raw_end() - raw_start()).max(ONE_MIN_MS);
                            let d = display_start().0;
                            let new_start = tz().local_to_utc_millis(d, h, m);
                            raw_start.set(new_start);
                            raw_end.set(new_start + duration);
                            emit();
                        },
                    }
                }

                div { class: "h-[0.5px] w-[15px] bg-text-secondary max-tablet:hidden" }

                div { class: "flex flex-row flex-1 gap-4 items-center min-w-0 @max-mobile:flex-col @max-mobile:w-full",
                    DatePicker {
                        selected_date: Some(display_end().0),
                        on_value_change: move |v: Option<Date>| {
                            if let Some(new_date) = v {
                                let (_, h, m) = display_end();
                                let new_end = tz().local_to_utc_millis(new_date, h, m);
                                raw_end.set(new_end.max(raw_start() + ONE_MIN_MS));
                                emit();
                            }
                        },
                        DatePickerInput { date: display_end().0.format(&format).unwrap_or_default() }
                    }
                    TimePicker {
                        hour: display_end().1,
                        minute: display_end().2,
                        on_change: move |(h, m)| {
                            let d = display_end().0;
                            let new_end = tz().local_to_utc_millis(d, h, m);
                            raw_end.set(new_end.max(raw_start() + ONE_MIN_MS));
                            emit();
                        },
                    }
                }

                TimezonePicker {
                    value: tz(),
                    on_change: move |new_tz: Timezone| {
                        tz.set(new_tz);
                        // Raw UTC ms don't change — display reactively
                        // re-renders via the memo. We still emit so the
                        // caller receives the new timezone.
                        emit();
                    },
                }
            }
        }
    }
}

#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    rsx! {
        div { class: "flex flex-1 min-w-0 @max-mobile:w-full",
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
        document::Stylesheet { href: asset!("./style.css") }
        div { class: "flex-1 w-full grow date-picker-group @max-mobile:w-full",
            DatePickerPopoverTrigger {
                div { class: "flex flex-row justify-between items-center w-full min-w-0 h-8 rounded-[8px] @max-mobile:w-full",
                    span { class: "grow", {date} }

                    icons::calendar::CalendarToday {
                        width: "20",
                        height: "20",
                        class: "border-[0px] shrink-0 text-icon-primary [&>path]:stroke-current [&>path]:fill-none [&>rect]:fill-current",
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
