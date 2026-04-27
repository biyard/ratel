use crate::common::models::space::SpaceUser;
use crate::features::spaces::pages::actions::actions::poll::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct VoteVerificationMaterialResponse {
    pub poll_id: String,
    pub voter_tag: String,
    pub voter_secret_key_json: String,
    pub ciphertext_hash: String,
    pub encrypted_vote_json: String,
    pub submitted_at_ms: i64,
}

#[get("/api/spaces/{space_pk}/polls/{poll_sk}/verification-material", role: SpaceUserRole, member: SpaceUser)]
pub async fn get_vote_verification_material(
    space_pk: SpacePartition,
    poll_sk: SpacePollEntityType,
) -> Result<VoteVerificationMaterialResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.into();

    let poll = SpacePoll::get(cli, &space_pk, Some(poll_sk_entity.clone()))
        .await?
        .ok_or(Error::NotFound("Poll not found".into()))?;
    if !poll.canister_upload_enabled {
        return Err(Error::NotFound(
            "Encrypted verification is not enabled for this poll".into(),
        ));
    }

    SpacePollUserAnswer::find_one(cli, &space_pk, &poll_sk_entity, &member.pk)
        .await?
        .ok_or(Error::NotFound("No vote found for this user".into()))?;

    use crate::features::spaces::pages::actions::services::vote_crypto::VOTE_CRYPTO_SERVICE;
    let crypto = VOTE_CRYPTO_SERVICE
        .as_ref()
        .ok_or(SpacePollError::VoteVerificationFailed)?;
    let voter_tag = crypto.build_voter_tag(&poll_sk_entity, &member.pk)?;
    let voter_secret_key_json = crypto.generate_voter_sk(&voter_tag)?;

    let canister = common_config.canister();
    let ballot = canister
        .get_ballot_by_tag(&poll_sk_entity.to_string(), &voter_tag)
        .await?
        .ok_or(Error::NotFound("No on-chain vote found".into()))?;
    let encrypted_vote_json = String::from_utf8(ballot.ciphertext_blob).map_err(|e| {
        crate::error!("Invalid canister ciphertext blob: {e}");
        SpacePollError::VoteVerificationFailed
    })?;

    Ok(VoteVerificationMaterialResponse {
        poll_id: poll_sk_entity.to_string(),
        voter_tag,
        voter_secret_key_json,
        ciphertext_hash: ballot.ciphertext_hash,
        encrypted_vote_json,
        submitted_at_ms: ballot.submitted_at_ms,
    })
}
