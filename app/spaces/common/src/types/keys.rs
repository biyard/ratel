use common::{SpacePartition, SpacePollEntityType};

pub type QueryKey = Vec<String>;

pub const SPACE_QUERY_KEY: &str = "Space";
pub const SPACE_USER_ROLE_QUERY_KEY: &str = "UserRole";
pub const SPACE_PAGE_ACTIONS_QUERY_KEY: &str = "Actions";
pub const SPACE_PAGE_ACTIONS_POLL_QUERY_KEY: &str = "Poll";
pub const SPACE_PAGE_DASHBOARD_QUERY_KEY: &str = "Dashboard";

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

pub fn space_page_dashboard_key(space_id: &SpacePartition) -> QueryKey {
    let mut k = space_key(space_id);
    k.push(SPACE_PAGE_DASHBOARD_QUERY_KEY.into());
    k
}
