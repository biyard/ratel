use crate::*;
use icons::game::Thunder;

pub fn get_nav_item(
    space_id: SpacePartition,
    _role: SpaceUserRole,
) -> Option<(Element, SpacePage, NavigationTarget)> {
    Some((
        icon(),
        SpacePage::Actions,
        Route::ListActionPage { space_id }.into(),
    ))
}

#[component]
pub fn icon() -> Element {
    rsx! {
        Thunder {}
    }
}
