use std::str::FromStr;

use easy_dynamodb::Document;
use dto::{ServiceError, AssemblyMember, District};
use super::openapi::member::MemberTrait;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Member(AssemblyMember);

impl Document for Member {
    fn document_type() -> String {
        "assembly_member".to_string()
    }
}

impl<T: MemberTrait> TryFrom<(String, String, &str, &T)> for Member {
    type Error = ServiceError;
    fn try_from(
        (code, image_url, lang, member): (String, String, &str, &T),
    ) -> Result<Self, ServiceError> {
        let now = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64;
        let district = District::from_str_by_lang(member.district(), lang).map_err(|_| ServiceError::BadRequest)?;

        Ok(Member(AssemblyMember {
            id: format!("{}-{}", lang, code),
            code,
            r#type: Member::document_type(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            name: Some(member.name().to_string()),
            party: Some(member.party().to_string()),
            district: Some(district),
            image_url: Some(image_url),
            email: Some(member.email().to_string()),
            gsi1: format!("{}#{}", Member::document_type(), lang),
            // gsi2: "".to_string(),
        }))
    }
}