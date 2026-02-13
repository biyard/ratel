use crate::*;

mod creator_page;
mod i18n;
mod new;

use creator_page::*;
use i18n::*;

pub use new::*;

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let role = use_server_future(move || async move { SpaceUserRole::Creator })?;

    match role.value()().unwrap_or_default() {
        SpaceUserRole::Creator => rsx! {
            CreatorPage { space_id }
        },
        _ => rsx! {},
    }
}
