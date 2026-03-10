use crate::features::spaces::pages::actions::actions::discussion::*;

use views::{DiscussionMainPage, EditorPage};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/spaces/:space_id/actions/discussions/:discussion_id")]
        #[route("/")]
        DiscussionMainPage { space_id: SpacePartition, discussion_id: SpacePostEntityType },
        #[route("/edit")]
        EditorPage { space_id: SpacePartition, discussion_id: SpacePostEntityType },
}
