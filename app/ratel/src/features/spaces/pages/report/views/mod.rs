use crate::features::spaces::pages::report::*;

mod candidate_page;
mod creator_page;
mod i18n;
mod participant_page;
mod viewer_page;

use candidate_page::*;
use creator_page::*;
use i18n::*;
use participant_page::*;
use viewer_page::*;

#[component]
pub fn SpaceReportPage(space_id: SpacePartition) -> Element {
    unimplemented!();
}
