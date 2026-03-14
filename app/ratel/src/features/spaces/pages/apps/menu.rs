use super::*;

// Space Layout Menu
pub fn get_nav_item(
    space_id: SpacePartition,
    role: SpaceUserRole,
) -> Option<(Element, SpacePage, NavigationTarget)> {
    if role != SpaceUserRole::Creator {
        return None;
    }
    Some((
        rsx! {
            icons::layouts::Apps {
                width: "20",
                height: "20",
                class: "text-icon-primary [&>path]:stroke-current",
            }
        },
        SpacePage::Apps,
        Route::SpaceAppsPage { space_id }.into(),
    ))
}
