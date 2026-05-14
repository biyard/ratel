use crate::common::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

/// Per-child marker pointing at the parent's anchor announcement Post.
///
/// **Why this exists:** earlier implementations cloned the parent's Post
/// onto every recognized sub-team's feed via `Post::create`, which
/// produced one Post row per (child × announcement). That broke every
/// natural invariant the platform assumed about Posts — same announcement
/// rendered with different `/posts/{uuid}` URLs depending on which team
/// you were on, likes / comments / shares scattered across rows, the
/// parent admin couldn't see the reception their own broadcast received,
/// and the wall list count diverged from the detail-page count.
///
/// Now there is **one anchor Post** (`pk = Feed(announcement_id)`,
/// authored by the parent team) and one of these markers per recognized
/// child. Child wall queries (`list_team_posts_handler`) read their own
/// Posts plus their fanout markers, batch-get the anchor Posts the
/// markers point at, and union the result. Every reader — parent admin,
/// every child — sees the same anchor URL, same likes / comments / shares,
/// same content. Direct messages use the same marker pattern with a
/// single targeted child.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubTeamAnnouncementFanout {
    /// `Partition::Team(child_team_id)` — the child team whose wall
    /// should surface this announcement.
    pub pk: Partition,
    /// `EntityType::SubTeamAnnouncementFanout(announcement_id)` — unique
    /// per (child, announcement) so re-publishing or re-fanout of the
    /// same announcement is idempotent.
    pub sk: EntityType,

    /// Anchor Post pk in string form (`"FEED#{announcement_id}"`).
    /// String rather than `Partition` so DynamoDB stores a single
    /// scalar attribute we can plug straight into a batch_get key.
    pub anchor_post_pk: String,

    /// Raw announcement id, mirroring the anchor Post's inner
    /// `announcement_id`. Indexed inline (no separate field) — keeps
    /// the wall query from having to re-parse the sk.
    pub announcement_id: String,

    /// Raw parent team UUID. Lets the wall renderer (and the
    /// draft-space filter in `list_team_posts_handler`) reason about
    /// parent identity without re-resolving the anchor Post.
    pub parent_team_id: String,

    /// True when the source announcement was a direct message
    /// (`target_child_team_id.is_some()`). Lets the wall surface a
    /// "direct" badge later without a second lookup.
    pub is_direct: bool,

    /// Timestamp of the anchor Post — used to order the marker
    /// alongside the child's own posts in the wall.
    pub created_at: i64,

    pub updated_at: i64,
}

#[cfg(feature = "server")]
impl SubTeamAnnouncementFanout {
    /// Build a marker row pointing at the parent's anchor Post.
    pub fn new(
        child_team_pk: Partition,
        announcement_id: String,
        parent_team_id: String,
        is_direct: bool,
        anchor_created_at: i64,
    ) -> Self {
        let anchor_post_pk = Partition::Feed(announcement_id.clone()).to_string();
        Self {
            pk: child_team_pk,
            sk: EntityType::SubTeamAnnouncementFanout(announcement_id.clone()),
            anchor_post_pk,
            announcement_id,
            parent_team_id,
            is_direct,
            created_at: anchor_created_at,
            updated_at: anchor_created_at,
        }
    }
}
