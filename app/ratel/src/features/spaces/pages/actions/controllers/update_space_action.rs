use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateSpaceActionRequest {
    Credits { credits: u64 },
    Time { started_at: i64, ended_at: i64 },
    Prerequisite { prerequisite: bool },
}

#[post("/api/spaces/{space_id}/actions/{action_id}", role: SpaceUserRole)]
pub async fn update_space_action(
    space_id: SpacePartition,
    action_id: String,
    req: UpdateSpaceActionRequest,
) -> Result<String> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let pk = CompositePartition(space_id, action_id);
    let sk = EntityType::SpaceAction;

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut updater =
        crate::features::spaces::pages::actions::models::SpaceAction::updater(&pk, &sk)
            .with_updated_at(now);

    match req {
        UpdateSpaceActionRequest::Credits { credits } => {
            let boost_multiplier = credits;
            let total_reward = credits * 10_000;
            updater = updater
                .with_credits(credits)
                .with_boost_multiplier(boost_multiplier)
                .with_total_reward(total_reward);
        }
        UpdateSpaceActionRequest::Time {
            started_at,
            ended_at,
        } => {
            if started_at >= ended_at {
                return Err(Error::BadRequest("Invalid time range".into()));
            }
            updater = updater.with_started_at(started_at).with_ended_at(ended_at);
        }
        UpdateSpaceActionRequest::Prerequisite { prerequisite } => {
            updater = updater.with_prerequisite(prerequisite);
        }
    }

    updater
        .execute(cli)
        .await
        .map_err(|e| Error::InternalServerError(format!("Failed to update space action: {e:?}")))?;

    Ok("success".to_string())
}
