use crate::common::types::{EntityType, Error, Partition};
use attr_voting::{
    authority::VotingAuthority,
    types::{UserAttributes, VotePayload},
    vote::{encrypt_vote, EncryptedVote},
};
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::features::spaces::pages::actions::actions::poll::types::Answer;
use ratel_canister::types::poll::QuestionVote;

type HmacSha256 = Hmac<Sha256>;

/// Envelope returned by `encrypt_poll_answers`, containing the encrypted
/// vote ciphertext, its hash (for on-chain dedup), and the blinded voter tag.
pub struct EncryptedVoteEnvelope {
    pub ciphertext_json: String,
    pub ciphertext_hash: String,
    pub voter_tag: String,
}

/// Build a blinded voter tag via HMAC-SHA256 so that the on-chain record
/// cannot be linked back to the user's raw ID.
pub fn build_voter_tag(poll_sk: &EntityType, user_pk: &Partition) -> Result<String, Error> {
    let secret = std::env::var("VOTER_TAG_SECRET").map_err(|_| {
        Error::InternalServerError("VOTER_TAG_SECRET not configured".to_string())
    })?;

    let user_id = user_inner_id(user_pk);
    let message = format!("{}:{}", poll_sk, user_id);

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|e| Error::InternalServerError(format!("HMAC init error: {}", e)))?;
    mac.update(message.as_bytes());
    let result = mac.finalize().into_bytes();

    use base64::Engine;
    Ok(base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(result))
}

/// Encrypt poll answers using CP-ABE. The resulting ciphertext can be
/// decrypted by keys with either the `ratel-authority` attribute or the
/// specific `voter-{voter_tag}` attribute.
pub fn encrypt_poll_answers(
    poll_sk: &EntityType,
    user_pk: &Partition,
    answers: &[Answer],
    submitted_at_ms: i64,
) -> Result<EncryptedVoteEnvelope, Error> {
    let authority_json = std::env::var("ATTR_VOTING_AUTHORITY_JSON").map_err(|_| {
        Error::InternalServerError("ATTR_VOTING_AUTHORITY_JSON not configured".to_string())
    })?;

    let authority = VotingAuthority::from_json(&authority_json)
        .map_err(|e| Error::InternalServerError(format!("Authority parse error: {}", e)))?;

    let voter_tag = build_voter_tag(poll_sk, user_pk)?;

    let choice = serde_json::to_string(answers)
        .map_err(|e| Error::InternalServerError(format!("Answer serialize error: {}", e)))?;

    let payload = VotePayload {
        choice,
        metadata: Some(serde_json::json!({
            "poll_sk": poll_sk.to_string(),
            "submitted_at_ms": submitted_at_ms,
        })),
    };

    let encrypted = encrypt_vote(&authority.public_key, &voter_tag, &payload)
        .map_err(|e| Error::InternalServerError(format!("ABE encrypt error: {}", e)))?;

    let ciphertext_json = serde_json::to_string(&encrypted.ciphertext)
        .map_err(|e| Error::InternalServerError(format!("Ciphertext serialize error: {}", e)))?;

    use sha2::Digest;
    let hash = sha2::Sha256::digest(ciphertext_json.as_bytes());
    let ciphertext_hash = hex::encode(hash);

    Ok(EncryptedVoteEnvelope {
        ciphertext_json,
        ciphertext_hash,
        voter_tag,
    })
}

/// Generate a voter secret key for the given blinded voter tag, and serialize
/// it to JSON so it can be stored in DynamoDB.
pub fn generate_voter_sk(voter_tag: &str) -> Result<String, Error> {
    let authority_json = std::env::var("ATTR_VOTING_AUTHORITY_JSON").map_err(|_| {
        Error::InternalServerError("ATTR_VOTING_AUTHORITY_JSON not configured".to_string())
    })?;

    let authority = VotingAuthority::from_json(&authority_json)
        .map_err(|e| Error::InternalServerError(format!("Authority parse error: {}", e)))?;

    let attrs = UserAttributes::voter(voter_tag);
    let sk = authority
        .generate_user_key(&attrs)
        .map_err(|e| Error::InternalServerError(format!("Keygen error: {}", e)))?;

    VotingAuthority::serialize_key(&sk)
        .map_err(|e| Error::InternalServerError(format!("SK serialize error: {}", e)))
}

/// Convert poll `Answer` values into `QuestionVote` entries for on-chain storage.
/// Each selected option becomes a separate `QuestionVote` entry.
pub fn answers_to_question_votes(
    answers: &[Answer],
    envelope: &EncryptedVoteEnvelope,
    submitted_at_ms: i64,
) -> Vec<QuestionVote> {
    let ciphertext_blob = envelope.ciphertext_json.as_bytes().to_vec();
    let mut votes = Vec::new();

    for (q_idx, answer) in answers.iter().enumerate() {
        let option_indices = answer_to_option_indices(answer);
        for opt_idx in option_indices {
            votes.push(QuestionVote {
                question_index: q_idx as u32,
                option_index: opt_idx,
                ciphertext_hash: envelope.ciphertext_hash.clone(),
                ciphertext_blob: ciphertext_blob.clone(),
                voter_tag: envelope.voter_tag.clone(),
                submitted_at_ms,
            });
        }
    }

    votes
}

/// Extract the selected option indices from an `Answer`.
fn answer_to_option_indices(answer: &Answer) -> Vec<u32> {
    match answer {
        Answer::SingleChoice { answer, .. } => {
            answer.map(|a| vec![a as u32]).unwrap_or_default()
        }
        Answer::MultipleChoice { answer, .. } => answer
            .as_ref()
            .map(|v| v.iter().map(|&a| a as u32).collect::<Vec<u32>>())
            .unwrap_or_default(),
        Answer::Checkbox { answer } => answer
            .as_ref()
            .map(|v| v.iter().map(|&a| a as u32).collect::<Vec<u32>>())
            .unwrap_or_default(),
        Answer::Dropdown { answer } => {
            answer.map(|a| vec![a as u32]).unwrap_or_default()
        }
        Answer::LinearScale { answer } => {
            answer.map(|a| vec![a as u32]).unwrap_or_default()
        }
        // Text answers are stored as option_index 0
        Answer::ShortAnswer { .. } | Answer::Subjective { .. } => vec![0],
    }
}

/// Extract the inner user ID from a `Partition`.
pub fn user_inner_id(user_pk: &Partition) -> String {
    match user_pk {
        Partition::User(id) => id.clone(),
        other => other.to_string(),
    }
}
