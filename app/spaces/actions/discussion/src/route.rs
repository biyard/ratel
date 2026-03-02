use crate::*;

use views::{EditorPage, ViewerPage};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/actions/discussions/:discussion_id")]
        #[route("/")]
        ViewerPage { space_id: SpacePartition, discussion_id: SpacePostEntityType },
        #[route("/edit")]
        EditorPage { space_id: SpacePartition, discussion_id: SpacePostEntityType },
}
