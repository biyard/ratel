use crate::features::spaces::pages::actions::actions::subscription::views::*;
use crate::features::spaces::pages::actions::actions::subscription::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/actions/subscriptions")]
        #[route("/")]
        MainPage { space_id: SpacePartition },
}
