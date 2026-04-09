use crate::common::*;
use crate::features::spaces::pages::actions::gamification::types::ChapterBenefit;
use crate::features::spaces::pages::actions::types::SpaceActionType;

/// Response payload for the Quest Map participant view endpoint
/// (`get_quest_map`). This DTO is the shape the frontend `QuestMap`
/// component consumes in Phase 4 of the gamification renewal.
///
/// The server computes each node's status, projected XP, and the
/// current user's state snapshot in a single pass so the client can
/// render the map without any additional round-trips. All ids are
/// serialized as raw `String` (no DynamoDB prefixes) so the payload
/// round-trips cleanly between the Axum server and the WASM client.
///
/// Chapters are ordered by `order` ascending, and each chapter's
/// `nodes` list contains every quest belonging to that chapter along
/// with its DAG dependencies (`depends_on`) so the client can draw
/// the node graph.

/// Lifecycle state of a single quest node as seen by the current
/// user. The server resolves this once per request based on DAG
/// parents, chapter completion, and the user's active role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuestNodeStatus {
    /// One or more DAG parents are not yet cleared — the quest is
    /// not reachable yet.
    Locked,
    /// All prerequisites are cleared and the quest is currently
    /// actionable by the user.
    Active,
    /// The user already submitted this quest successfully.
    Cleared,
    /// Prerequisites are cleared but the user's role is below the
    /// chapter's `actor_role`, so they can see the node but cannot
    /// act on it yet.
    RoleGated,
}

/// Result of a cleared quiz node, surfaced back to the client so the
/// Quest Map can render the user's score badge. Only populated for
/// cleared quiz nodes; `None` for every other node type and for
/// uncleared quizzes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestQuizResult {
    /// Number of correct answers the user submitted.
    pub score: i64,
    /// Total number of quiz questions.
    pub total: i64,
    /// Whether the submission met the pass threshold defined on the
    /// quiz action.
    pub passed: bool,
}

/// A single quest (poll / quiz / discussion / follow) as rendered on
/// the Quest Map. One `QuestNodeView` per `SpaceAction` that belongs
/// to a chapter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuestNodeView {
    /// Action id (raw, no `SPACE_ACTION#` prefix).
    pub id: String,
    /// Discriminator for which kind of action this node represents.
    pub action_type: SpaceActionType,
    /// Display title of the quest.
    pub title: String,
    /// Flat base XP reward configured on the action.
    pub base_points: i64,
    /// Server-computed XP at stake for this node given the current
    /// participant count, combo multiplier, and streak multiplier
    /// (`base × participants × combo × streak`). This is the number
    /// the UI should display as the "reward" to the user.
    pub projected_xp: i64,
    /// Lifecycle state of the node for the requesting user.
    pub status: QuestNodeStatus,
    /// Action ids this node depends on in the chapter DAG. Every
    /// parent must be `Cleared` before this node can leave `Locked`.
    pub depends_on: Vec<String>,
    /// Owning chapter id (raw, no prefix).
    pub chapter_id: String,
    /// Optional quest start timestamp (unix seconds).
    pub started_at: Option<i64>,
    /// Optional quest end timestamp (unix seconds).
    pub ended_at: Option<i64>,
    /// Quiz result summary — only populated for cleared quiz nodes;
    /// `None` otherwise.
    pub quiz_result: Option<QuestQuizResult>,
}

/// A single chapter on the Quest Map, together with its owned quest
/// nodes and the user's progress inside it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChapterView {
    /// Chapter id (raw, no `SPACE_CHAPTER#` prefix).
    pub id: String,
    /// 1-based display order inside the space.
    pub order: u32,
    /// Chapter display name.
    pub name: String,
    /// Optional longer description shown in the chapter header.
    pub description: Option<String>,
    /// Minimum role a user must hold to act on quests inside this
    /// chapter. Users below this role see nodes as `RoleGated`.
    pub actor_role: SpaceUserRole,
    /// What the user receives when every quest in the chapter is
    /// cleared (role upgrade, XP bonus, or none).
    pub completion_benefit: ChapterBenefit,
    /// Quests that belong to this chapter.
    pub nodes: Vec<QuestNodeView>,
    /// True if every node in this chapter is `Cleared` for the
    /// current user.
    pub is_complete: bool,
    /// Best-estimate sum of `projected_xp` across cleared nodes in
    /// this chapter. Used by the UI to show "XP earned so far".
    pub total_xp_earned: i64,
}

/// Snapshot of the requesting user's per-space gamification state.
/// Returned alongside the chapter list so the UI can render the
/// active chapter indicator and multiplier badges without another
/// request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UserQuestState {
    /// The user's current role in this space.
    pub role: SpaceUserRole,
    /// First chapter that is not yet complete and whose role/prior
    /// gates allow the user to act on it. `None` once the user has
    /// cleared every chapter.
    pub current_chapter_id: Option<String>,
    /// Combo multiplier currently applied when computing
    /// `QuestNodeView::projected_xp`.
    pub current_combo_multiplier: f32,
    /// Number of consecutive days the user has submitted an action
    /// in this space, feeding the streak multiplier.
    pub current_streak_days: u32,
}

/// Top-level Quest Map response returned by `get_quest_map`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct QuestMapResponse {
    /// Chapters sorted by `order` ascending. Each chapter carries
    /// its own list of quest nodes.
    pub chapters: Vec<ChapterView>,
    /// Per-user snapshot used to render the current-chapter marker
    /// and multiplier badges.
    pub current_user_state: UserQuestState,
}
