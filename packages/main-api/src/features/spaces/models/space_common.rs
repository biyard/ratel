use ssi::claims::ResourceProvider;

use crate::features::spaces::members::{InvitationStatus, SpaceInvitationMember};
use crate::features::spaces::panels::{SpacePanelParticipant, SpacePanelQuota, SpacePanels};
use crate::*;
use crate::{
    Error,
    features::spaces::members::SpaceEmailVerification,
    models::{User, UserTeamGroup, UserTeamGroupQueryOption, team::Team, *},
    types::*,
    utils::time::get_now_timestamp_millis,
};

use super::SpaceParticipant;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]

pub struct SpaceCommon {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi2", sk)]
    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi6", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    pub status: Option<SpaceStatus>, // Waiting, InProgress, Started, Finished

    #[dynamo(
        prefix = "SPACE_COMMON_VIS",
        name = "find_by_visibility",
        index = "gsi6",
        order = 2,
        pk
    )]
    pub visibility: SpaceVisibility, // Private, Public, Team(team_pk)
    #[dynamo(index = "gsi6", order = 1, pk)]
    pub publish_state: SpacePublishState, // Draft, Published
    #[dynamo(prefix = "POST_PK", name = "find_by_post_pk", index = "gsi2", pk)]
    pub post_pk: Partition,
    pub space_type: SpaceType,
    #[serde(default)]
    pub content: String,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,

    pub booster: BoosterType,
    pub custom_booster: Option<i64>,
    pub rewards: Option<i64>,

    #[serde(default)]
    pub reports: i64,

    #[serde(default)]
    pub anonymous_participation: bool,
    //FIXME: Remove this field.
    //We already have publish_state to control the visibility of the space.
    #[serde(default)]
    pub change_visibility: bool,
    #[serde(default)]
    // participants is the number of participants. It is incremented when a user participates in the space.
    // It is only used for spaces enabling explicit participation such as anonymous participation.
    pub participants: i64,

    // space pdf files
    pub files: Option<Vec<File>>,

    #[serde(default)]
    pub block_participate: bool,

    #[serde(default = "max_quota")]
    pub quota: i64,
    #[serde(default = "max_quota")]
    pub remains: i64,
}

fn max_quota() -> i64 {
    1_000_000
}

impl SpaceCommon {
    pub fn new(
        Post {
            pk: post_pk,
            user_pk,
            author_display_name,
            author_profile_url,
            author_username,
            ..
        }: Post,
    ) -> Self {
        let now = get_now_timestamp_millis();
        Self {
            pk: post_pk
                .clone()
                .to_space_pk()
                .expect("post_pk must be Partition::Feed"),
            sk: EntityType::SpaceCommon,
            created_at: now,
            updated_at: now,
            post_pk,
            publish_state: SpacePublishState::Draft,
            status: None,
            visibility: SpaceVisibility::Private,
            user_pk,
            author_display_name,
            author_profile_url,
            author_username,
            ..Default::default()
        }
    }

    pub fn should_explicit_participation(&self) -> bool {
        self.anonymous_participation
    }

    pub fn is_published(&self) -> bool {
        self.publish_state == SpacePublishState::Published
    }

    pub async fn is_space_admin(&self, cli: &aws_sdk_dynamodb::Client, user: &User) -> bool {
        if matches!(&self.user_pk, Partition::User(_)) {
            &self.user_pk == &user.pk
        } else if matches!(&self.user_pk, Partition::Team(_)) {
            Team::has_permission(cli, &self.user_pk, &user.pk, TeamGroupPermission::TeamAdmin)
                .await
                .unwrap_or(false)
        } else {
            false
        }
    }
}

impl SpaceCommon {
    pub fn permissions_for_guest(&self) -> TeamGroupPermissions {
        if self.visibility == SpaceVisibility::Public
            && self.publish_state == SpacePublishState::Published
        {
            return TeamGroupPermissions::read();
        }

        TeamGroupPermissions::empty()
    }

    pub fn validate_editable(&self) -> bool {
        self.publish_state == SpacePublishState::Draft
            || (self.publish_state == SpacePublishState::Published
                && (self.status == Some(SpaceStatus::Waiting) || self.status.is_none()))
    }

    pub async fn get_participant(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
    ) -> Result<Option<SpaceParticipant>> {
        let (pk, sk) = SpaceParticipant::keys(self.pk.clone(), user_pk.clone());

        SpaceParticipant::get(cli, &pk, Some(&sk)).await
    }
    pub async fn check_if_satisfying_panel_attribute(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        user: &User,
    ) -> Result<()> {
        let panel_quota = SpacePanelQuota::query(
            cli,
            CompositePartition(self.pk.clone(), Partition::PanelAttribute),
            SpacePanelQuota::opt_all().sk("SPACE_PANEL_ATTRIBUTE#".to_string()),
        )
        .await
        .unwrap_or_default()
        .0;

        if panel_quota.is_empty() {
            return Ok(());
        }

        let user_attributes = user.get_attributes(cli).await?;
        let age: Option<u8> = user_attributes.age().and_then(|v| u8::try_from(v).ok());
        let gender = user_attributes.gender;

        if self.remains <= 0 {
            return Err(Error::LackOfVerifiedAttributes);
        }

        for q in panel_quota {
            if q.remains <= 0 {
                continue;
            }

            if let EntityType::SpacePanelAttribute(label, _) = &q.sk {
                if label.eq_ignore_ascii_case("university") {
                    continue;
                }
            }

            if match_by_sk(age, gender.clone(), &q.sk) {
                let pk = q.pk;
                let sk = q.sk;

                let (panel_pk, panel_sk) =
                    SpacePanelParticipant::keys(&self.pk.clone(), &user.pk.clone());

                let participant =
                    SpacePanelParticipant::get(cli, panel_pk, Some(panel_sk.clone())).await?;

                if participant.is_none() {
                    let participants = SpacePanelParticipant::new(self.pk.clone(), user.clone());

                    let space_updater =
                        SpaceCommon::updater(self.pk.clone(), EntityType::SpaceCommon)
                            .decrease_remains(1);

                    let quota_updater =
                        SpacePanelQuota::updater(pk.clone(), sk.clone()).decrease_remains(1);

                    transact_write!(
                        cli,
                        participants.create_transact_write_item(),
                        space_updater.transact_write_item(),
                        quota_updater.transact_write_item(),
                    )?;
                }

                return Ok(());
            }
        }

        Err(Error::LackOfVerifiedAttributes)
    }
}
#[async_trait::async_trait]
impl ResourcePermissions for SpaceCommon {
    fn viewer_permissions(&self) -> Permissions {
        if self.visibility == SpaceVisibility::Public
            && self.publish_state == SpacePublishState::Published
        {
            return Permissions::read();
        }

        Permissions::empty()
    }

    fn participant_permissions(&self) -> Permissions {
        Permissions::read()
    }

    fn resource_owner(&self) -> ResourceOwnership {
        self.user_pk.clone().into()
    }

    async fn is_participant(&self, cli: &aws_sdk_dynamodb::Client, requester: &Partition) -> bool {
        self.get_participant(cli, requester)
            .await
            .map(|sp| sp.is_some())
            .unwrap_or(false)
    }

    async fn can_participate(&self, cli: &aws_sdk_dynamodb::Client, requester: &Partition) -> bool {
        let (pk, sk) = SpaceInvitationMember::keys(&self.pk, requester);
        SpaceInvitationMember::get(cli, &pk, Some(&sk))
            .await
            .map(|sp: Option<SpaceInvitationMember>| {
                if let Some(sp) = sp {
                    sp.status == InvitationStatus::Invited
                        || sp.status == InvitationStatus::Accepted
                } else {
                    false
                }
            })
            .unwrap_or(false)
    }
}

pub fn match_by_sk(age: Option<u8>, gender: Option<Gender>, sk: &EntityType) -> bool {
    if age.is_none() && gender.is_none() {
        return false;
    }

    let (label_raw, value_raw) = match sk {
        EntityType::SpacePanelAttribute(label, value) => (label.as_str(), value.as_str()),
        _ => return false,
    };

    let label = label_raw.to_ascii_lowercase();
    let value = value_raw.to_ascii_lowercase();

    match label.as_str() {
        "verifiable_attribute" => match value.as_str() {
            v if v.starts_with("age") => match_age_rule(age, v),
            v if v.starts_with("gender") => match_gender_rule(gender, v),
            _ => false,
        },
        "collective_attribute" => true,
        "gender" => {
            let encoded = format!("gender:{}", value);
            match_gender_rule(gender, &encoded)
        }
        "university" => true,

        _ => false,
    }
}

fn match_age_rule(age: Option<u8>, v: &str) -> bool {
    if v == "age" {
        return age.is_some();
    }

    if let Some(rest) = v.strip_prefix("age:") {
        if let Some((min_s, max_s)) = rest.split_once('-') {
            if let (Ok(min), Ok(max)) = (min_s.trim().parse::<u8>(), max_s.trim().parse::<u8>()) {
                return age.map(|a| a >= min && a <= max).unwrap_or(false);
            }
        } else if let Ok(specific) = rest.trim().parse::<u8>() {
            return age.map(|a| a == specific).unwrap_or(false);
        }
    }

    true
}

fn match_gender_rule(gender: Option<Gender>, v: &str) -> bool {
    if v == "gender" {
        return gender.is_some();
    }

    // "gender:male" | "gender:female"
    if let Some(rest) = v.strip_prefix("gender:") {
        let want = rest.trim().to_ascii_lowercase();
        return match (want.as_str(), gender) {
            ("male", Some(Gender::Male)) => true,
            ("female", Some(Gender::Female)) => true,
            _ => false,
        };
    }

    true
}
