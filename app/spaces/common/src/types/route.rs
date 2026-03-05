use common::{SpacePartition, SpacePollEntityType, SpacePostEntityType};

pub fn space_root(space_id: &SpacePartition) -> String {
    format!("/spaces/{space_id}/dashboard")
}

pub fn space_overview(space_id: &SpacePartition) -> String {
    format!("/spaces/{space_id}/overview")
}

pub fn space_actions(space_id: &SpacePartition) -> String {
    format!("/spaces/{space_id}/actions")
}

pub fn space_action_poll(space_id: &SpacePartition, poll_id: &SpacePollEntityType) -> String {
    format!("/spaces/{space_id}/actions/polls/{poll_id}")
}

pub fn space_action_discussion(
    space_id: &SpacePartition,
    discussion_id: &SpacePostEntityType,
) -> String {
    format!("/spaces/{space_id}/actions/discussions/{discussion_id}")
}

pub fn space_action_discussion_edit(
    space_id: &SpacePartition,
    discussion_id: &SpacePostEntityType,
) -> String {
    format!("/spaces/{space_id}/actions/discussions/{discussion_id}/edit")
}

pub fn space_apps(space_id: &SpacePartition) -> String {
    format!("/spaces/{space_id}/apps")
}

pub fn space_app_main(space_id: &SpacePartition) -> String {
    format!("/spaces/{space_id}/apps")
}

pub fn space_app_general(space_id: &SpacePartition) -> String {
    format!("/spaces/{space_id}/apps/general")
}

pub fn space_app_incentive_pool(space_id: &SpacePartition) -> String {
    format!("/spaces/{space_id}/apps/incentive_pool")
}
