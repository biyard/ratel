use easy_dynamodb::Document;
use dto::{ServiceError, AssemblyMember};
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

        Ok(Member(AssemblyMember {
            id: format!("{}-{}", lang, code),
            code,
            r#type: Member::document_type(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            name: Some(member.name().to_string()),
            party: Some(member.party().to_string()),
            district: Some(member.district().to_string()),
            image_url: Some(image_url),
            gsi1: lang.to_string(),
            // gsi2: "".to_string(),
        }))
    }
}