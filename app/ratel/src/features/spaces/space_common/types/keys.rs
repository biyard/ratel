use crate::common::{SpacePartition, SpacePollEntityType, SpacePostEntityType, SpaceQuizEntityType};

pub type QueryKey = Vec<String>;

pub const SPACE_QUERY_KEY: &str = "Space";
pub const SPACE_USER_ROLE_QUERY_KEY: &str = "UserRole";
pub const SPACE_PAGE_ACTIONS_QUERY_KEY: &str = "Actions";
pub const SPACE_PAGE_ACTIONS_POLL_QUERY_KEY: &str = "Poll";
pub const SPACE_PAGE_ACTIONS_DISCUSSION_QUERY_KEY: &str = "Discussion";
pub const SPACE_PAGE_ACTIONS_DISCUSSION_COMMENTS_QUERY_KEY: &str = "Comments";
pub const SPACE_PAGE_ACTIONS_QUIZ_QUERY_KEY: &str = "Quiz";
pub const SPACE_PAGE_DASHBOARD_QUERY_KEY: &str = "Dashboard";
pub const SPACE_RANKING_QUERY_KEY: &str = "Ranking";
pub const SPACE_MY_SCORE_QUERY_KEY: &str = "MyScore";

pub fn space_key(space_id: &SpacePartition) -> QueryKey {
    vec![SPACE_QUERY_KEY.into(), space_id.to_string()]
}

pub fn space_user_role_key(space_id: &SpacePartition) -> QueryKey {
    let mut k = space_key(space_id);
    k.push(SPACE_USER_ROLE_QUERY_KEY.into());
    k
}

pub fn space_page_actions_key(space_id: &SpacePartition) -> QueryKey {
    let mut k = space_key(space_id);
    k.push(SPACE_PAGE_ACTIONS_QUERY_KEY.into());
    k
}

pub fn space_page_actions_poll_key(
    space_id: &SpacePartition,
    poll_sk: &SpacePollEntityType,
) -> QueryKey {
    let mut k = space_page_actions_key(space_id);
    k.push(SPACE_PAGE_ACTIONS_POLL_QUERY_KEY.into());
    k.push(poll_sk.to_string());
    k
}

pub fn space_page_actions_discussion_key(
    space_id: &SpacePartition,
    discussion_sk: &SpacePostEntityType,
) -> QueryKey {
    let mut k = space_page_actions_key(space_id);
    k.push(SPACE_PAGE_ACTIONS_DISCUSSION_QUERY_KEY.into());
    k.push(discussion_sk.to_string());
    k
}

pub fn space_page_actions_discussion_comments_key(
    space_id: &SpacePartition,
    discussion_sk: &SpacePostEntityType,
) -> QueryKey {
    let mut k = space_page_actions_discussion_key(space_id, discussion_sk);
    k.push(SPACE_PAGE_ACTIONS_DISCUSSION_COMMENTS_QUERY_KEY.into());
    k
}

pub fn space_page_actions_quiz_key(
    space_id: &SpacePartition,
    quiz_sk: &SpaceQuizEntityType,
) -> QueryKey {
    let mut k = space_page_actions_key(space_id);
    k.push(SPACE_PAGE_ACTIONS_QUIZ_QUERY_KEY.into());
    k.push(quiz_sk.to_string());
    k
}

pub fn space_page_dashboard_key(space_id: &SpacePartition) -> QueryKey {
    let mut k = space_key(space_id);
    k.push(SPACE_PAGE_DASHBOARD_QUERY_KEY.into());
    k
}

pub fn space_ranking_key(space_id: &SpacePartition) -> QueryKey {
    let mut k = space_key(space_id);
    k.push(SPACE_RANKING_QUERY_KEY.into());
    k
}

pub fn space_my_score_key(space_id: &SpacePartition) -> QueryKey {
    let mut k = space_key(space_id);
    k.push(SPACE_MY_SCORE_QUERY_KEY.into());
    k
}
