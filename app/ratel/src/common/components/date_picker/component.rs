use dioxus::prelude::*;

use dioxus_primitives::{
    date_picker::{
        self, DatePickerInputProps as OriginDatePickerInputProps, DatePickerProps,
        DateRangePickerProps,
    },
    popover::{PopoverContentProps, PopoverTriggerProps},
    ContentAlign,
};
use time::{format_description, Date};

use crate::{Input, InputVariant, Separator};

use super::super::calendar::*;
use super::super::popover::*;

#[component]
pub fn DateAndTimePicker() -> Element {
    let mut selected_start_date = use_signal(|| None::<Date>);
    let mut selected_end_date = use_signal(|| None::<Date>);
    let format = format_description::parse("[year]-[month]-[day]").unwrap();

    rsx! {
        div { class: "flex flex-row gap-4 items-center w-full",
            DatePicker {
                selected_date: selected_start_date(),
                on_value_change: move |v| {
                    selected_start_date.set(v);
                },
                DatePickerInput { date: selected_start_date().and_then(|d| d.format(&format).ok()).unwrap_or_default() }
            }
            input {
                r#type: "time",
                class: "flex flex-row flex-1 gap-1 justify-between items-center self-stretch bg-input-bg-regular border-input-stroker-regular rounded-[12px] border-[0.5px] p-[0.5em]",
            }

            div { class: "h-[0.5px] w-[15px] bg-text-secondary" }

            DatePicker {
                selected_date: selected_end_date(),
                on_value_change: move |v| {
                    selected_end_date.set(v);
                },
                DatePickerInput { date: selected_end_date().and_then(|d| d.format(&format).ok()).unwrap_or_default() }
            }

            input {
                r#type: "time",
                class: "flex flex-row flex-1 gap-1 justify-between items-center self-stretch bg-input-bg-regular border-[0.5px] border-input-stroker-regular rounded-[12px] p-[0.5em]",
            }
        }
    }
}

#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    rsx! {
        div {
            date_picker::DatePicker {
                class: "flex-1 date-picker",
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
                date_picker::DatePickerPopover { popover_root: PopoverRoot, {props.children} }
            }
        }
    }
}

#[component]
pub fn DateRangePicker(props: DateRangePickerProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
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
        div { class: "flex-1 date-picker-group bg-input-bg-regular border-input-stroker-regular border-[0.5px] p-[0.5em] rounded-[12px]",
            DatePickerPopoverTrigger {
                label { class: "flex flex-row justify-between items-center w-full min-w-0 h-8 rounded-[8px]",
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
