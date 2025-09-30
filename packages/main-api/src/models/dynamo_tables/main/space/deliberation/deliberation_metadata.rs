use crate::{
    models::space::SpaceCommon,
    types::{EntityType, Partition, SpaceVisibility},
};

use super::*;
use bdk::prelude::*;
use std::{collections::HashMap, mem};

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct DeliberationResponse {
    pub pk: Partition,
    pub created_at: i64,
    pub updated_at: i64,
    pub likes: i64,
    pub comments: i64,
    pub rewards: i64,
    pub shares: i64,
    pub visibility: SpaceVisibility,
    pub title: String,
    pub post_pk: Partition,
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
}

impl From<DeliberationSpace> for DeliberationResponse {
    fn from(deliberation: DeliberationSpace) -> Self {
        Self {
            pk: deliberation.pk,
            created_at: deliberation.created_at,
            updated_at: deliberation.updated_at,
            likes: 0,
            comments: 0,
            rewards: 0,
            shares: 0,
            visibility: SpaceVisibility::Public,
            title: "".to_string(),
            post_pk: Partition::Feed("".to_string()),
            user_pk: deliberation.user_pk,
            author_display_name: deliberation.author_display_name,
            author_profile_url: deliberation.author_profile_url,
            author_username: deliberation.author_username,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct DeliberationDetailResponse {
    #[serde(flatten)]
    pub deliberation: DeliberationResponse,
    pub summary: DeliberationContentResponse,
    pub discussions: Vec<DeliberationDiscussionResponse>,
    pub elearnings: ElearningResponse,
    pub surveys: DeliberationSurveyResponse,
    pub recommendation: DeliberationContentResponse,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum DeliberationMetadata {
    DeliberationSpace(DeliberationSpace),
    DeliberationSpaceSurvey(DeliberationSpaceSurvey),
    DeliberationSpaceResponse(DeliberationSpaceResponse),
    DeliberationSpaceContent(DeliberationSpaceContent),
    DeliberationSpaceQuestion(DeliberationSpaceQuestion),
    DeliberationSpaceParticipant(DeliberationSpaceParticipant),
    DeliberationSpaceMember(DeliberationSpaceMember),
    DeliberationSpaceElearning(DeliberationSpaceElearning),
    DeliberationSpaceDiscussion(DeliberationSpaceDiscussion),
    SpaceCommon(SpaceCommon),
}

fn discussion_id_of(pk: &Partition) -> String {
    if let Partition::Discussion(v) = pk {
        v.clone()
    } else {
        String::new()
    }
}

fn participant_resp_from_dsp(p: &DeliberationSpaceParticipant) -> DiscussionParticipantResponse {
    DiscussionParticipantResponse {
        user_pk: p.user_pk.clone(),
        author_display_name: p.author_display_name.clone(),
        author_profile_url: p.author_profile_url.clone(),
        author_username: p.author_username.clone(),
        participant_id: p.participant_id.clone().unwrap_or_default(),
    }
}
fn member_resp_from_dsp(p: &DeliberationSpaceParticipant) -> DiscussionMemberResponse {
    DiscussionMemberResponse {
        user_pk: p.user_pk.clone(),
        author_display_name: p.author_display_name.clone(),
        author_profile_url: p.author_profile_url.clone(),
        author_username: p.author_username.clone(),
    }
}
fn participant_resp_from_dsm(m: &DeliberationSpaceMember) -> DiscussionParticipantResponse {
    DiscussionParticipantResponse {
        user_pk: m.user_pk.clone(),
        author_display_name: m.author_display_name.clone(),
        author_profile_url: m.author_profile_url.clone(),
        author_username: m.author_username.clone(),
        participant_id: String::new(),
    }
}
fn member_resp_from_dsm(m: &DeliberationSpaceMember) -> DiscussionMemberResponse {
    DiscussionMemberResponse {
        user_pk: m.user_pk.clone(),
        author_display_name: m.author_display_name.clone(),
        author_profile_url: m.author_profile_url.clone(),
        author_username: m.author_username.clone(),
    }
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
                DeliberationMetadata::DeliberationSpaceContent(content) => match content.sk {
                    EntityType::DeliberationSpaceSummary => {
                        res.summary = DeliberationContentResponse {
                            html_contents: content.html_contents,
                            files: content.files,
                        };
                    }
                    EntityType::DeliberationSpaceRecommendation => {
                        res.recommendation = DeliberationContentResponse {
                            html_contents: content.html_contents,
                            files: content.files,
                        };
                    }
                    _ => continue,
                },
                DeliberationMetadata::DeliberationSpaceResponse(response) => {
                    let response: SurveyResponseResponse = response.into();
                    res.surveys.responses.push(response);
                }
                DeliberationMetadata::DeliberationSpaceQuestion(question) => {
                    res.surveys.questions = question.question();
                }
                DeliberationMetadata::DeliberationSpaceParticipant(participant) => {
                    match participant.sk {
                        EntityType::DeliberationSpaceParticipant(_) => {
                            participants_by_discussion
                                .entry(discussion_id_of(&participant.discussion_pk))
                                .or_default()
                                .push(participant_resp_from_dsp(&participant));
                        }
                        EntityType::DeliberationSpaceMember(_) => {
                            members_by_discussion
                                .entry(discussion_id_of(&participant.discussion_pk))
                                .or_default()
                                .push(member_resp_from_dsp(&participant));
                        }
                        _ => {}
                    }
                }
                DeliberationMetadata::DeliberationSpaceMember(member) => match member.sk {
                    EntityType::DeliberationSpaceParticipant(_) => {
                        participants_by_discussion
                            .entry(discussion_id_of(&member.discussion_pk))
                            .or_default()
                            .push(participant_resp_from_dsm(&member));
                    }
                    EntityType::DeliberationSpaceMember(_) => {
                        members_by_discussion
                            .entry(discussion_id_of(&member.discussion_pk))
                            .or_default()
                            .push(member_resp_from_dsm(&member));
                    }
                    _ => {}
                },
                DeliberationMetadata::DeliberationSpaceElearning(deliberation_space_elearning) => {
                    res.elearnings = deliberation_space_elearning.into();
                }
                DeliberationMetadata::DeliberationSpaceDiscussion(discussion) => {
                    let disc: DeliberationDiscussionResponse = discussion.into();
                    res.discussions.push(disc);
                }
                DeliberationMetadata::SpaceCommon(space_common) => {
                    res.deliberation.visibility = space_common.visibility;
                    res.deliberation.post_pk = space_common.post_pk;
                }
            }
        }

        for disc in &mut res.discussions {
            let disc_pk = match &disc.pk {
                Partition::Discussion(v) => v.clone(),
                _ => String::new(),
            };

            if let Some(parts) = participants_by_discussion.remove(&disc_pk) {
                disc.participants = parts;
            }
            if let Some(mems) = members_by_discussion.remove(&disc_pk) {
                disc.members = mems;
            }
        }

        res
    }
}
