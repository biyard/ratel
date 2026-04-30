mod follower_setting;
pub use follower_setting::FollowerSetting;

mod i18n;
pub use i18n::FollowCreatorTranslate;

mod config_card;
mod targets_card;
use config_card::ConfigCard;
use targets_card::TargetsCard;

use crate::features::spaces::pages::actions::actions::follow::*;
use crate::features::spaces::pages::actions::actions::follow::controllers::get_follow;
use crate::features::spaces::pages::actions::components::{
    ActionEditFooter, ActionEditSaveBus, ActionEditTopbar,
};

#[component]
pub fn FollowCreatorPage(
    space_id: ReadSignal<SpacePartition>,
    follow_id: ReadSignal<SpaceActionFollowEntityType>,
) -> Element {
    let tr: FollowCreatorTranslate = use_translate();
    let action_setting =
        use_loader(move || async move { get_follow(space_id(), follow_id()).await })?;
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let nav = use_navigator();

    let initial_title = action_setting().title.clone();
    let title = use_signal(|| initial_title);

    ActionEditSaveBus::provide();
    let current_page = use_signal(|| 0usize);

    let action_for_signal = action_setting();
    let action_setting_signal: ReadSignal<
        crate::features::spaces::pages::actions::models::SpaceAction,
    > = use_signal(move || action_for_signal.clone()).into();

    rsx! {
        div { class: "arena",
            ActionEditTopbar {
                space_name: space.title.clone(),
                action_type_label: tr.type_badge_label.to_string(),
                action_type_key: "follow",
                title,
                on_title_change: move |_v: String| {},
                editable_title: false,
                on_back: move |_| {
                    nav.go_back();
                },
                on_cancel: move |_| {
                    nav.go_back();
                },
            }
            main { class: "pager",
                div {
                    class: "pager__track",
                    style: "transform: translateX(-{current_page() * 100}%);",
                    TargetsCard {
                        space_id,
                        follow_id,
                        initial_title: action_setting().title.clone(),
                    }
                    ConfigCard {
                        space_id,
                        follow_id,
                        action_setting: action_setting_signal,
                    }
                }
            }
            ActionEditFooter { current_page, total_pages: 2, action_type_key: "follow" }
        }
    }
}
