use crate::common::models::space::SpaceAuthor;
use crate::features::spaces::pages::actions::actions::poll::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyVoteResponse {
    pub voter_tag: String,
    pub ciphertext_hash: String,
    pub decrypted_choice: String,
    pub decrypted_metadata: Option<serde_json::Value>,
}

#[get("/api/spaces/{space_pk}/polls/{poll_sk}/verify", role: SpaceUserRole, author: SpaceAuthor)]
pub async fn verify_vote(
    space_pk: SpacePartition,
    poll_sk: SpacePollEntityType,
) -> Result<VerifyVoteResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.into();

    SpacePollUserAnswer::find_one(cli, &space_pk, &poll_sk_entity, &author.pk)
        .await?
        .ok_or(Error::NotFound("No vote found for this user".into()))?;

    use crate::features::spaces::pages::actions::services::vote_crypto::VOTE_CRYPTO_SERVICE;
    let crypto = VOTE_CRYPTO_SERVICE
        .as_ref()
        .ok_or(Error::InternalServerError(
            "Encrypted voting is not configured".into(),
        ))?;
    let voter_tag = crypto.build_voter_tag(&poll_sk_entity, &author.pk)?;

    let canister = common_config.canister();
    let ballot = canister
        .get_ballot_by_tag(&poll_sk_entity.to_string(), &voter_tag)
        .await?
        .ok_or(Error::NotFound("No on-chain vote found".into()))?;

    let decrypted = crypto.decrypt(&poll_sk_entity, &author.pk, &ballot.ciphertext_blob)?;

    if decrypted.ciphertext_hash != ballot.ciphertext_hash {
        return Err(Error::InternalServerError(
            "On-chain ciphertext hash mismatch".into(),
        ));
    }

    Ok(VerifyVoteResponse {
        voter_tag: decrypted.voter_tag,
        ciphertext_hash: decrypted.ciphertext_hash,
        decrypted_choice: decrypted.choice,
        decrypted_metadata: decrypted.metadata,
    })
}
