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
use crate::features::spaces::pages::actions::components::ActionEditTopbar;

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

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "arena",
            ActionEditTopbar {
                space_name: space.title.clone(),
                action_type_label: tr.type_badge_label.to_string(),
                action_type_key: "follow".to_string(),
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
                TargetsCard {
                    space_id,
                    follow_id,
                    initial_title: action_setting().title.clone(),
                }
                ConfigCard {
                    space_id,
                    follow_id,
                    credits: action_setting().credits,
                    prerequisite: action_setting().prerequisite,
                    action_status: action_setting().status.clone(),
                }
            }
        }
    }
}
