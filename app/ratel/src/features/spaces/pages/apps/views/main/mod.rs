use crate::features::spaces::pages::apps::*;

mod app_card;
mod creator_page;
mod i18n;
mod viewer_page;

use app_card::AppCard;

use crate::features::spaces::space_common::hooks::use_space_role;
use creator_page::CreatorPage;
use i18n::AppMainTranslate;
use viewer_page::ViewerPage;

#[component]
pub fn SpaceAppsPage(space_id: SpacePartition) -> Element {
    let role = use_space_role()();
    match role {
        SpaceUserRole::Creator => rsx! {
            CreatorPage { space_id }
        },
        _ => rsx! {
            ViewerPage { space_id }
        },
    }
}
