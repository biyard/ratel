#[cfg(feature = "server")]
use crate::models::UserAttributesExt;
use crate::models::{File, Gender, SpacePanelParticipant, SpacePanelQuota};
use crate::*;
use ratel_post::types::{BoosterType, SpacePublishState, SpaceStatus, SpaceType, SpaceVisibility};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(DynamoEntity))]
pub struct SpaceCommon {
    pub pk: Partition,
    pub sk: EntityType,
    #[cfg_attr(feature = "server", dynamo(index = "gsi2", sk))]
    #[cfg_attr(feature = "server", dynamo(index = "gsi1", sk))]
    #[cfg_attr(feature = "server", dynamo(index = "gsi6", sk))]
    pub created_at: i64,
    pub updated_at: i64,
    pub status: Option<SpaceStatus>,
    #[cfg_attr(
        feature = "server",
        dynamo(
            prefix = "SPACE_COMMON_VIS",
            name = "find_by_visibility",
            index = "gsi6",
            order = 2,
            pk
        )
    )]
    pub visibility: SpaceVisibility,
    #[cfg_attr(feature = "server", dynamo(index = "gsi6", order = 1, pk))]
    pub publish_state: SpacePublishState,
    #[cfg_attr(
        feature = "server",
        dynamo(prefix = "POST_PK", name = "find_by_post_pk", index = "gsi2", pk)
    )]
    pub post_pk: Partition,
    pub space_type: SpaceType,
    #[serde(default)]
    pub content: String,
    #[cfg_attr(
        feature = "server",
        dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)
    )]
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
    #[deprecated(note = "Use Visibility variant instead")]
    #[serde(default)]
    pub change_visibility: bool,
    #[serde(default)]
    pub participants: i64,
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

#[cfg(feature = "server")]
impl SpaceCommon {
    pub async fn check_if_satisfying_panel_attribute(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        user: &ratel_auth::User,
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
            return Err(Error::FullQuota);
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

            if match_by_sk(age, gender, &q.sk) {
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

#[cfg(feature = "server")]
fn match_by_sk(age: Option<u8>, gender: Option<Gender>, sk: &EntityType) -> bool {
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
            let encoded = format!("gender:{value}");
            match_gender_rule(gender, &encoded)
        }
        "university" => true,
        _ => false,
    }
}

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
fn match_gender_rule(gender: Option<Gender>, v: &str) -> bool {
    if v == "gender" {
        return gender.is_some();
    }

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
