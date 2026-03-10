use super::*;
use icons::game::Thunder;

pub fn get_nav_item(
    space_id: SpacePartition,
    _role: SpaceUserRole,
) -> Option<(Element, SpacePage, NavigationTarget)> {
    Some((
        icon(),
        SpacePage::Actions,
        Route::SpaceActionsPage { space_id }.into(),
    ))
}

#[component]
pub fn icon() -> Element {
    rsx! {
        Thunder {
            width: "20",
            height: "20",
            class: "[&>path]:fill-none [&>path]:stroke-[#737373]",
        }
    }
}
