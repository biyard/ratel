use super::*;
// use time::{ext::NumericalDuration, UtcDateTime};

#[component]
pub fn ActionCommonSettings() -> Element {
    let tr: ActionCommonSettingsTranslate = use_translate();
    let mut enable_reward = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col gap-5 w-full",
            div { class: "flex flex-col gap-2.5",
                p { {tr.date} }
                DateAndTimePicker {}
            }
            Card {
                direction: CardDirection::Row,
                main_axis_align: MainAxisAlign::Between,
                cross_axis_align: CrossAxisAlign::Center,
                p { {tr.reward_setting} }
                Switch {
                    active: enable_reward(),
                    on_toggle: move |_| {
                        enable_reward.set(!enable_reward());
                    },
                }
            }
        }
    }
}
use crate::*;

translate! {
    ActionCommonSettingsTranslate;

    date: {
        en: "Date",
        ko: "참여기간",
    },
    reward_setting: {
        en: "Reward Setting",
        ko: "보상 설정",
    },
}
