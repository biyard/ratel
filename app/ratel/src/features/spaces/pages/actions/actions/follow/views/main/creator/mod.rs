mod follower_setting;
use follower_setting::FollowerSetting;

use crate::features::spaces::pages::actions::actions::follow::controllers::get_follow;
use crate::features::spaces::pages::actions::actions::follow::*;

mod i18n;
use i18n::FollowCreatorTranslate;

#[component]
pub fn FollowCreatorPage(
    space_id: ReadSignal<SpacePartition>,
    follow_id: ReadSignal<SpaceActionFollowEntityType>,
) -> Element {
    let tr: FollowCreatorTranslate = use_translate();
    let action_setting =
        use_loader(move || async move { get_follow(space_id(), follow_id()).await })?;

    // The delete button is the only thing the lifecycle lock still
    // applies to — creators can keep editing the follower list and
    // common settings even after the action has started.
    let space = crate::features::spaces::space_common::hooks::use_space()();
    let locked = crate::features::spaces::pages::actions::is_action_locked(
        space.status,
        action_setting().started_at,
    );

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            h3 { {tr.title} }
            Tabs { default_value: "follower-tab",
                TabList {
                    TabTrigger { index: 0usize, value: "follower-tab", {tr.tab_general} }
                    TabTrigger { index: 1usize, value: "setting-tab", {tr.tab_common} }
                }
                TabContent { index: 0usize, value: "follower-tab",
                    FollowerSetting { space_id }
                }
                TabContent { index: 1usize, value: "setting-tab",
                    div { class: "flex flex-col gap-4 w-full",
                        ActionCommonSettings {
                            space_id,
                            action_id: follow_id().to_string(),
                            action_setting: action_setting(),
                        }
                        // Delete button is hidden once the action is locked.
                        if !locked {
                            ActionDeleteButton {
                                space_id: space_id(),
                                action_id: follow_id().to_string(),
                            }
                        }
                    }
                }
            }
        }
    }
}
