use crate::features::spaces::actions::shell::*;
use dioxus::router::components::child_router::ChildRouter;

use crate::features::spaces::actions::discussion::Route as DiscussionRoute;
use crate::features::spaces::actions::main::Route as MainRoute;
use crate::features::spaces::actions::poll::Route as PollRoute;
use crate::features::spaces::actions::quiz::Route as QuizRoute;
use crate::features::spaces::actions::subscription::Route as SubscriptionRoute;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/actions")]
        #[route("/polls/:..rest")]
        Poll { space_id: SpacePartition, rest: Vec<String> },
        #[route("/quizzes/:..rest")]
        Quiz { space_id: SpacePartition, rest: Vec<String> },
        #[route("/discussions/:..rest")]
        Discussion { space_id: SpacePartition, rest: Vec<String> },
        #[route("/subscriptions/:..rest")]
        Subscription { space_id: SpacePartition, rest: Vec<String> },
        #[route("/:..rest")]
        Main { space_id: SpacePartition, rest: Vec<String> },

}

macro_rules! define_action_route_wrapper {
    ($wrapper_name:ident, $route_ty:ty) => {
        #[component]
        fn $wrapper_name(space_id: SpacePartition, rest: Vec<String>) -> Element {
            let router = use_context::<dioxus::router::RouterContext>();
            let route: $route_ty = router.current();
            rsx! {
                ChildRouter::<$route_ty> {
                    route,
                    format_route_as_root_route: |r: $route_ty| r.to_string(),
                    parse_route_from_root_route: |url: &str| {
                        <$route_ty as std::str::FromStr>::from_str(url).ok()
                    },
                }
            }
        }
    };
}

define_action_route_wrapper!(Main, MainRoute);
define_action_route_wrapper!(Poll, PollRoute);
define_action_route_wrapper!(Quiz, QuizRoute);
define_action_route_wrapper!(Discussion, DiscussionRoute);
define_action_route_wrapper!(Subscription, SubscriptionRoute);
