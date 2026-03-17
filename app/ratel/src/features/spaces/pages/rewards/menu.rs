use crate::features::spaces::pages::apps::apps::rewards::controllers::list_space_rewards;
use crate::features::spaces::space_common::controllers::SpaceResponse;
use crate::*;

pub fn get_nav_item(
    space: &SpaceResponse,
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
        Route::SpaceRewardsPage {
            space_id: space.id.clone(),
        }
        .into(),
    ))
}
