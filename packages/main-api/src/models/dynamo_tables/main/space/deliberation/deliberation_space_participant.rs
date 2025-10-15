use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct DeliberationSpaceParticipant {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", sk)]
    #[dynamo(index = "gsi6", sk)]
    pub sk: EntityType,

    pub participant_id: Option<String>,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    #[serde(default)]
    #[dynamo(
        prefix = "DISCUSSION_USER_PK",
        name = "find_by_discussion_user_pk",
        index = "gsi6",
        pk
    )]
    pub discussion_user_pk: Option<Partition>,

    #[dynamo(
        prefix = "DISCUSSION_PK",
        name = "find_by_discussion_pk",
        index = "gsi2",
        pk
    )]
    pub discussion_pk: Partition,
}

//FIXME: GSI6 is unnecessary.
// We know DISCUSSION_PK + USER_ID is unique.
// So we can use PK: PARTICIPANT#USER#UID, SK:DISCUSSION_PK(DISCUSSION#UID)
// And We can found multiple participant of DISCUSSION_PK with (GSI1_PK : SK, GSI1_SK: created_at)
// DISCUSSION_PK + USER_ID 는 유일합니다.
// 그래서 우리는 PK: PARTICIPANT#{USER_PK}, SK:{DISCUSSION_PK} 로 구성할 수 있습니다.
// 특정 유저의 참석 여부를 알기 위해서는 PK, SK 로 조회 가능합니다.
// 특정 DISCUSSION_PK 에 속한 모든 참석자를 찾기 위해서는 GSI1 (GSI1_PK: SK, GSI1_SK: created_at) 으로 조회 가능합니다.

//FIXME: PK 를 Deliberation_PK 와 동일하게 가져가면 안됩니다.
// 동일한 PK를 가진 Entity 중 1:N 관계를 가지는 경우, 1MB가 넘는 경우에는 조회가 불가능해집니다.
// 그래서 DISCUSSION_PK 등으로 DELIBERTAION_PK 와 분리해야합니다.

impl DeliberationSpaceParticipant {
    pub fn new(
        deliberation_pk: Partition,
        discussion_pk: Partition,

        participant_id: String,
        User {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        let discussion_id = match &discussion_pk {
            Partition::Discussion(v) => v.as_str(),
            _ => "",
        };
        let user_id = match &pk {
            Partition::User(v) | Partition::Team(v) => v.as_str(),
            _ => "",
        };

        Self {
            pk: deliberation_pk,
            sk: EntityType::DeliberationDiscussionParticipant(
                discussion_id.to_string(),
                participant_id.clone(),
            ),
            participant_id: Some(participant_id),
            discussion_user_pk: Some(Partition::DiscussionUser(format!(
                "{discussion_id}#{user_id}"
            ))),
            user_pk: pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,
            discussion_pk,
        }
    }

    pub fn id(&self) -> String {
        if let Some(id) = &self.participant_id {
            return id.clone();
        }
        if let EntityType::DeliberationDiscussionParticipant(_, v) = &self.sk {
            return v.clone();
        }
        String::new()
    }

    pub fn discussion_user_pk_or_compute(&self) -> Partition {
        if let Some(pk) = &self.discussion_user_pk {
            return pk.clone();
        }
        let discussion_id = match &self.discussion_pk {
            Partition::Discussion(v) => v.as_str(),
            _ => "",
        };
        let user_id = match &self.user_pk {
            Partition::User(v) | Partition::Team(v) => v.as_str(),
            _ => "",
        };
        Partition::DiscussionUser(format!("{discussion_id}#{user_id}"))
    }
}

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct DiscussionParticipantResponse {
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
    pub participant_id: String,
}

impl From<DeliberationSpaceParticipant> for DiscussionParticipantResponse {
    fn from(p: DeliberationSpaceParticipant) -> Self {
        Self {
            user_pk: p.clone().user_pk,
            author_display_name: p.clone().author_display_name,
            author_profile_url: p.clone().author_profile_url,
            author_username: p.clone().author_username,
            participant_id: p.id(),
        }
    }
}
