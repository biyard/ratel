use crate::{
    models::space::SpaceCommon,
    types::{Partition, SpaceVisibility},
};

use super::*;
use bdk::prelude::*;
use std::{collections::HashMap, mem};

#[derive(Debug, Clone, Default, serde::Serialize, schemars::JsonSchema)]
pub struct DeliberationResponse {
    pub pk: String,

    pub created_at: i64,
    pub updated_at: i64,

    pub likes: i64,
    pub comments: i64,
    pub rewards: i64,
    pub shares: i64,

    pub visibility: SpaceVisibility,
    pub title: String,

    pub post_pk: String,
    pub user_pk: String,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
}

impl From<DeliberationSpace> for DeliberationResponse {
    fn from(deliberation: DeliberationSpace) -> Self {
        let pk = match deliberation.pk {
            Partition::DeliberationSpace(v) => v,
            _ => "".to_string(),
        };

        let user_pk = match deliberation.user_pk {
            Partition::User(v) => v,
            Partition::Team(v) => v,
            _ => "".to_string(),
        };

        Self {
            pk,
            created_at: deliberation.created_at,
            updated_at: deliberation.updated_at,
            //FIXME: fix to this line when post is implemented
            likes: 0,
            comments: 0,
            rewards: 0,
            shares: 0,
            visibility: SpaceVisibility::Public,
            title: "".to_string(),
            post_pk: "".to_string(),

            user_pk,
            author_display_name: deliberation.author_display_name,
            author_profile_url: deliberation.author_profile_url,
            author_username: deliberation.author_username,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, JsonSchema)]
pub struct DeliberationDetailResponse {
    #[serde(flatten)]
    pub deliberation: DeliberationResponse,

    pub summary: DeliberationSummaryResponse,
    pub discussions: Vec<DeliberationDiscussionResponse>,
    pub elearnings: ElearningResponse,
    pub surveys: DeliberationSurveyResponse,

    pub recommendation: DeliberationRecommentationResponse,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum DeliberationMetadata {
    DeliberationSpace(DeliberationSpace),
    DeliberationSpaceSurvey(DeliberationSpaceSurvey),
    DeliberationSpaceSummary(DeliberationSpaceSummary),
    DeliberationSpaceResponse(DeliberationSpaceResponse),
    DeliberationSpaceRecommendation(DeliberationSpaceRecommendation),
    DeliberationSpaceQuestion(DeliberationSpaceQuestion),
    DeliberationSpaceParticipant(DeliberationSpaceParticipant),
    DeliberationSpaceMember(DeliberationSpaceMember),
    DeliberationSpaceElearning(DeliberationSpaceElearning),
    DeliberationSpaceDiscussion(DeliberationSpaceDiscussion),
    SpaceCommon(SpaceCommon),
}

impl From<Vec<DeliberationMetadata>> for DeliberationDetailResponse {
    fn from(items: Vec<DeliberationMetadata>) -> Self {
        let mut res = Self::default();

        let mut participants_by_discussion: HashMap<String, Vec<DiscussionParticipantResponse>> =
            HashMap::new();
        let mut members_by_discussion: HashMap<String, Vec<DiscussionMemberResponse>> =
            HashMap::new();

        for item in items {
            match item {
                DeliberationMetadata::DeliberationSpace(deliberation_space) => {
                    let prev = mem::replace(&mut res.deliberation, deliberation_space.into());
                    res.deliberation.post_pk = prev.post_pk;
                    res.deliberation.visibility = prev.visibility;
                }
                DeliberationMetadata::DeliberationSpaceSurvey(survey) => {
                    let prev = mem::replace(&mut res.surveys, survey.into());

                    res.surveys.questions = prev.questions;
                    res.surveys.responses = prev.responses;
                    res.surveys.user_responses = prev.user_responses;
                }
                DeliberationMetadata::DeliberationSpaceSummary(deliberation_space_summary) => {
                    res.summary = deliberation_space_summary.into();
                }
                DeliberationMetadata::DeliberationSpaceResponse(response) => {
                    let response: SurveyResponseResponse = response.into();
                    res.surveys.responses.push(response);
                    //FIXME: add user response
                }
                DeliberationMetadata::DeliberationSpaceRecommendation(
                    deliberation_space_recommendation,
                ) => {
                    res.recommendation = deliberation_space_recommendation.into();
                }
                DeliberationMetadata::DeliberationSpaceQuestion(question) => {
                    res.surveys.questions = question.question();
                }
                DeliberationMetadata::DeliberationSpaceParticipant(participant) => {
                    let discussion_id = match &participant.discussion_pk {
                        Partition::Discussion(v) => v.clone(),
                        _ => String::new(),
                    };

                    let participant = participant.into();
                    participants_by_discussion
                        .entry(discussion_id)
                        .or_default()
                        .push(participant);
                }
                DeliberationMetadata::DeliberationSpaceMember(member) => {
                    let discussion_id = match &member.discussion_pk {
                        Partition::Discussion(v) => v.clone(),
                        _ => String::new(),
                    };
                    let member = member.into();
                    members_by_discussion
                        .entry(discussion_id)
                        .or_default()
                        .push(member);
                }
                DeliberationMetadata::DeliberationSpaceElearning(deliberation_space_elearning) => {
                    res.elearnings = deliberation_space_elearning.into();
                }
                DeliberationMetadata::DeliberationSpaceDiscussion(discussion) => {
                    let disc: DeliberationDiscussionResponse = discussion.into();

                    res.discussions.push(disc);
                }
                DeliberationMetadata::SpaceCommon(space_common) => {
                    res.deliberation.visibility = space_common.visibility;
                    res.deliberation.post_pk = match space_common.post_pk {
                        Partition::Feed(v) => v,
                        _ => "".to_string(),
                    };
                }
            }
        }

        for disc in &mut res.discussions {
            if let Some(parts) = participants_by_discussion.remove(&disc.pk) {
                disc.participants = parts;
            }
            if let Some(mems) = members_by_discussion.remove(&disc.pk) {
                disc.members = mems;
            }
        }

        res
    }
}
