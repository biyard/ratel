use crate::features::spaces::pages::actions::actions::poll::*;

use views::MainPage;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/actions/polls/:poll_id")]
        #[route("/")]
        MainPage { space_id: SpacePartition, poll_id: SpacePollEntityType },
}
