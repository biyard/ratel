use super::*;
// use time::{ext::NumericalDuration, UtcDateTime};

#[component]
pub fn ActionCommonSettings() -> Element {
    // let now = UtcDateTime::now().date();

    // let mut selected_range = use_signal(|| {
    //     let now = UtcDateTime::now().date();
    //     Some(DateRange::new(now, now.saturating_add(3.days())))
    // });

    rsx! {
        div { DateAndTimePicker {} }
    }
}
