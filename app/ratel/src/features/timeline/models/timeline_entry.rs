use crate::features::timeline::*;

/// Reason why a post appeared in someone's timeline.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum TimelineReason {
    /// Author posted it and you follow them
    #[default]
    Following,
    /// You are a member of the team that posted it
    TeamMember,
    /// Post is trending (met popularity criteria)
    Popular,
}

impl std::fmt::Display for TimelineReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimelineReason::Following => write!(f, "following"),
            TimelineReason::TeamMember => write!(f, "team_member"),
            TimelineReason::Popular => write!(f, "popular"),
        }
    }
}

impl std::str::FromStr for TimelineReason {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "following" => Ok(TimelineReason::Following),
            "team_member" => Ok(TimelineReason::TeamMember),
            "popular" => Ok(TimelineReason::Popular),
            _ => Err(format!("unknown timeline reason: {}", s)),
        }
    }
}

/// All available timeline categories.
pub const TIMELINE_CATEGORIES: &[TimelineReason] = &[
    TimelineReason::Following,
    TimelineReason::TeamMember,
    TimelineReason::Popular,
];

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TimelineEntry {
    pub pk: Partition,  // TIMELINE#{user_id}
    pub sk: EntityType, // TIMELINE_ENTRY#{timestamp}#{post_pk_inner}

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,

    pub post_pk: Partition,   // FEED#{uuid} - reference to actual post
    pub author_pk: Partition, // USER#{author_id} or TEAM#{team_id}
    pub reason: TimelineReason,

    /// GSI1 pk: TL_CAT#{user_id}#{reason}
    /// Enables querying timeline entries by category (Netflix-style rows).
    #[dynamo(prefix = "TL_CAT", name = "find_by_category", index = "gsi1", pk)]
    pub category_key: String,
}

#[cfg(feature = "server")]
impl TimelineEntry {
    pub fn new(
        user_pk: &Partition,
        post_pk: &Partition,
        author_pk: &Partition,
        created_at: i64,
        reason: TimelineReason,
    ) -> Self {
        let user_id = match user_pk {
            Partition::User(id) => id.clone(),
            Partition::Team(id) => id.clone(),
            _ => user_pk.to_string(),
        };

        let post_pk_inner = match post_pk {
            Partition::Feed(id) => id.clone(),
            _ => post_pk.to_string(),
        };

        let category_key = format!("{}#{}", user_id, reason);

        Self {
            pk: Partition::Timeline(user_id),
            sk: EntityType::TimelineEntry(format!("{}#{}", created_at, post_pk_inner)),
            created_at,
            post_pk: post_pk.clone(),
            author_pk: author_pk.clone(),
            reason,
            category_key,
        }
    }
}
