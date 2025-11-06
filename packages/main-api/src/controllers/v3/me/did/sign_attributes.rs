use crate::features::did::*;
use crate::services::portone::PortOne;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum SignAttributesRequest {
    #[schemars(description = "Sign attributes via PortOne")]
    PortOne {
        #[schemars(description = "Identity Verifycation ID")]
        id: String,
    },
}

pub async fn sign_attributes_handler(
    State(AppState {
        portone, dynamo, ..
    }): State<AppState>,
    NoApi(user): NoApi<User>,
    Json(req): Json<SignAttributesRequest>,
) -> Result<Json<AttributeIssuanceResponseV2>> {
    tracing::debug!("Handling request: {:?}", req);

    match req {
        SignAttributesRequest::PortOne { id } => {
            portone_sign_attributes_handler(&portone, &dynamo.client, &user, id).await
        }
    }
}

async fn portone_sign_attributes_handler(
    portone: &PortOne,
    _ddb: &aws_sdk_dynamodb::Client,
    user: &User,
    id: String,
) -> Result<Json<AttributeIssuanceResponseV2>> {
    // Get identity verification data from PortOne
    let identity_result = portone.identify(&id).await?;

    tracing::info!(
        "Identity verification retrieved for user {}: {:?}",
        user.username,
        identity_result.verified_customer
    );

    // Get configuration
    let config = crate::config::get();
    let domain = &config.domain;
    let bbs_key = config.did.bbs_bls_key;

    // Get issuer DID (platform DID) and subject DID (user DID)
    let issuer_did = ssi::dids::DIDBuf::from_string(format!("did:web:{}", domain))
        .map_err(|e| Error::InternalServerError(format!("Invalid issuer DID: {}", e)))?;
    let subject_did = get_did(&user.username)?;

    // Create attribute signer with BBS+ key for selective disclosure
    let signer = AttributeSigner::new(issuer_did, subject_did, bbs_key);

    // Calculate age from birth_date
    let birth_date = &identity_result.verified_customer.birth_date;
    let age = calculate_age_from_birth_date(birth_date)?;

    // Get gender
    let gender = identity_result.verified_customer.gender.to_string();

    // Sign attributes with BBS+ signatures
    let attributes = vec![
        ("age", serde_json::json!(age)),
        ("gender", serde_json::json!(gender)),
    ];

    let issuance_response = signer
        .sign_attributes(attributes, Some(365))
        .map_err(|e| Error::InternalServerError(format!("Failed to sign attributes: {}", e)))?;

    tracing::info!(
        "Successfully signed verifiable credential for user {}",
        user.username
    );

    Ok(Json(issuance_response))
}

/// Calculate age from birth_date string (format: YYYYMMDD)
fn calculate_age_from_birth_date(birth_date: &str) -> Result<u32> {
    use chrono::{Datelike, Utc};

    if birth_date.len() != 8 {
        return Err(Error::BadRequest(
            "Invalid birth date format. Expected YYYYMMDD".into(),
        ));
    }

    let year = birth_date[0..4]
        .parse::<i32>()
        .map_err(|_| Error::BadRequest("Invalid birth year".into()))?;
    let month = birth_date[4..6]
        .parse::<u32>()
        .map_err(|_| Error::BadRequest("Invalid birth month".into()))?;
    let day = birth_date[6..8]
        .parse::<u32>()
        .map_err(|_| Error::BadRequest("Invalid birth day".into()))?;

    let now = Utc::now();
    let current_year = now.year();
    let current_month = now.month();
    let current_day = now.day();

    let mut age = (current_year - year) as u32;

    // Adjust if birthday hasn't occurred yet this year
    if current_month < month || (current_month == month && current_day < day) {
        age -= 1;
    }

    Ok(age)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Utc};

    #[test]
    fn test_calculate_age_from_birth_date() {
        let now = Utc::now();
        let current_year = now.year();

        // Test someone born 30 years ago
        let birth_date = format!("{}0101", current_year - 30);
        let age = calculate_age_from_birth_date(&birth_date).unwrap();
        assert!(age >= 29 && age <= 30); // Depends on current date

        // Test someone born 25 years ago in December
        let birth_date = format!("{}1231", current_year - 25);
        let age = calculate_age_from_birth_date(&birth_date).unwrap();
        assert!(age >= 24 && age <= 25);
    }

    #[test]
    fn test_calculate_age_invalid_format() {
        let result = calculate_age_from_birth_date("199001");
        assert!(result.is_err());

        let result = calculate_age_from_birth_date("19900101123");
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_age_invalid_values() {
        let result = calculate_age_from_birth_date("abcd0101");
        assert!(result.is_err());

        let result = calculate_age_from_birth_date("1990ab01");
        assert!(result.is_err());

        let result = calculate_age_from_birth_date("199001ab");
        assert!(result.is_err());
    }
}
