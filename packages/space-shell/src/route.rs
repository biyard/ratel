use crate::*;
use actions::App as ActionsApp;
use apps::App as AppsApp;
use dashboard::App as DashboardApp;
use overview::App as OverviewApp;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id")]
    #[layout(SpaceLayout)]
    #[route("/dashboard/:..rest")]
    DashboardApp { space_id: SpacePartition, rest: Vec<String> },
    #[route("/overview/:..rest")]
    OverviewApp { space_id: SpacePartition, rest: Vec<String> },
    #[route("/actions/:..rest")]
    ActionsApp { space_id: SpacePartition, rest: Vec<String> },
    #[route("/apps/:..rest")]
    AppsApp { space_id: SpacePartition, rest: Vec<String> },

}
