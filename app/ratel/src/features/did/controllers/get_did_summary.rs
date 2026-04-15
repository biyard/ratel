use crate::features::did::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct DidAttributeItem {
    /// Stable attribute key. Known values: "age", "gender", "university",
    /// "employer", "membership".
    pub key: String,
    /// Verification method: "kyc" (PortOne) or "code" (offline code).
    pub method: String,
    /// Present only when verified.
    pub value: Option<String>,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct DidSummaryResponse {
    pub did: String,
    pub issued_at: i64,
    /// Proxy for "most recent credential update": taken from the user record's
    /// `updated_at` since VerifiedAttributes does not track its own timestamps.
    pub last_verified_at: Option<i64>,
    pub attributes: Vec<DidAttributeItem>,
    pub verified_count: usize,
    pub total_slots: usize,
}

#[get("/api/me/did/summary", user: crate::features::auth::User)]
pub async fn get_did_summary_handler() -> Result<DidSummaryResponse> {
    use crate::common::models::did::VerifiedAttributes;

    let conf = crate::common::CommonConfig::default();
    let cli = conf.dynamodb();

    let pk = CompositePartition(user.pk.clone(), Partition::Attributes);
    let attrs = VerifiedAttributes::get(cli, pk, None::<String>)
        .await?
        .unwrap_or_default();

    let age = attrs.age();
    let gender_value = attrs.gender.as_ref().map(|g| g.to_string());
    let university = attrs.university.clone();
    let has_any = age.is_some() || gender_value.is_some() || university.is_some();

    let mut items = Vec::<DidAttributeItem>::new();
    items.push(DidAttributeItem {
        key: "age".to_string(),
        method: "kyc".to_string(),
        value: age.map(|n| n.to_string()),
        verified: age.is_some(),
    });
    items.push(DidAttributeItem {
        key: "gender".to_string(),
        method: "kyc".to_string(),
        value: gender_value.clone(),
        verified: gender_value.is_some(),
    });
    items.push(DidAttributeItem {
        key: "university".to_string(),
        method: "code".to_string(),
        value: university.clone(),
        verified: university.is_some(),
    });
    items.push(DidAttributeItem {
        key: "employer".to_string(),
        method: "code".to_string(),
        value: None,
        verified: false,
    });
    items.push(DidAttributeItem {
        key: "membership".to_string(),
        method: "code".to_string(),
        value: None,
        verified: false,
    });

    let verified_count = items.iter().filter(|i| i.verified).count();
    let total_slots = items.len();

    Ok(DidSummaryResponse {
        did: user.did(),
        issued_at: user.created_at,
        last_verified_at: if has_any { Some(user.updated_at) } else { None },
        attributes: items,
        verified_count,
        total_slots,
    })
}
