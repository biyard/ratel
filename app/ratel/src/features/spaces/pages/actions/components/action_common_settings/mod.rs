mod reward_setting;
pub use reward_setting::*;

use super::*;

#[component]
pub fn ActionCommonSettings(
    #[props(default)] on_date_change: EventHandler<DateTimeRange>,
) -> Element {
    let tr: ActionCommonSettingsTranslate = use_translate();

    rsx! {
        div { class: "flex flex-col gap-5 w-full",
            div { class: "flex flex-col gap-2.5",
                p { {tr.date} }
                DateAndTimePicker { on_change: move |range| on_date_change.call(range) }
            }

            RewardSetting {}
        }
    }
}

translate! {
    ActionCommonSettingsTranslate;

    date: {
        en: "Date",
        ko: "참여기간",
    },
}
