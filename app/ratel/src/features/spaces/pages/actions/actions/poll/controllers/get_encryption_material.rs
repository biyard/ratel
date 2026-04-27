use crate::common::models::space::SpaceUser;
use crate::features::spaces::pages::actions::actions::poll::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct VoteEncryptionMaterialResponse {
    pub poll_id: String,
    pub voter_tag: String,
    pub voter_secret_key_json: String,
    pub authority_public_key_json: String,
}

#[get("/api/spaces/{space_pk}/polls/{poll_sk}/encryption-material", role: SpaceUserRole, member: SpaceUser)]
pub async fn get_encryption_material(
    space_pk: SpacePartition,
    poll_sk: SpacePollEntityType,
) -> Result<VoteEncryptionMaterialResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.into();

    let poll = SpacePoll::get(cli, &space_pk, Some(poll_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Poll not found".into()))?;
    if !poll.canister_upload_enabled {
        return Err(Error::NotFound(
            "Encrypted voting is not enabled for this poll".into(),
        ));
    }

    use crate::features::spaces::pages::actions::services::vote_crypto::VOTE_CRYPTO_SERVICE;
    let crypto = VOTE_CRYPTO_SERVICE
        .as_ref()
        .ok_or(SpacePollError::EncryptionFailed)?;

    let voter_tag = crypto.build_voter_tag(&poll_sk_entity, &member.pk)?;
    let voter_secret_key_json = crypto.generate_voter_sk(&voter_tag)?;
    let authority_public_key_json = crypto.authority_public_key_json()?;

    Ok(VoteEncryptionMaterialResponse {
        poll_id: poll_sk_entity.to_string(),
        voter_tag,
        voter_secret_key_json,
        authority_public_key_json,
    })
}
