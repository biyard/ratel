use crate::features::spaces::pages::apps::apps::rewards::controllers::list_space_rewards;
use crate::*;

pub fn get_nav_item(
    space_id: SpacePartition,
    role: SpaceUserRole,
) -> Option<(Element, SpacePage, NavigationTarget)> {
    if role != SpaceUserRole::Participant {
        return None;
    }

    Some((
        rsx! {
            icons::game::Trophy {
                width: "20",
                height: "20",
                class: "text-icon-primary [&>path]:stroke-current",
            }
        },
        SpacePage::Rewards,
        Route::SpaceRewardsPage { space_id }.into(),
    ))
}
