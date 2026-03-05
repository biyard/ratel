use crate::*;

mod app_card;
mod creator_page;
mod i18n;
mod viewer_page;

use app_card::AppCard;

use creator_page::CreatorPage;
use i18n::AppMainTranslate;
use space_common::hooks::use_user_role;
use viewer_page::ViewerPage;

#[component]
pub fn AppMainPage(space_id: SpacePartition) -> Element {
    let role_loader = use_user_role(&space_id)?;
    let role = role_loader.read().clone();
    match role {
        SpaceUserRole::Creator => rsx! {
            CreatorPage { space_id }
        },
        _ => rsx! {
            ViewerPage { space_id }
        },
    }
}
