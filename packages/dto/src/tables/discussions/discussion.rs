use bdk::prelude::*;
use validator::Validate;

use crate::*;

#[derive(Validate)]
#[api_model(base = "/v1/spaces/:space-id/discussions", table = discussions, action = [create(participants = Vec<i64>)], action_by_id = [start_meeting, participant_meeting, exit_meeting, start_recording, end_recording, delete])]
pub struct Discussion {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = spaces)]
    pub space_id: i64,

    #[api_model(summary, version = v0.1)]
    pub creator_id: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub started_at: i64,
    #[api_model(summary, action = create, action_by_id = update)]
    pub ended_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub name: String,

    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,

    #[api_model(summary, action_by_id = update)]
    pub meeting_id: Option<String>,

    #[api_model(summary, action_by_id = update)]
    pub pipeline_id: String,

    #[api_model(summary, many_to_many = discussion_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = discussion_id)]
    #[serde(default)]
    pub members: Vec<Member>,
    #[api_model(summary, one_to_many = discussion_participants, foreign_key = discussion_id)]
    #[serde(default)]
    pub participants: Vec<DiscussionParticipant>,
}
