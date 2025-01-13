use serde::{Serialize, Deserialize};
use crate::ServiceError;
use dioxus_translate::Language;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionAssemblyMemberRequest {
    /// Fetches assembly members by Assembly Open API.
    /// And update the information of the assembly members.
    FetchMembers,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionAssemblyMemberByIdRequest {
    /// Manually, update crypto stance.
    /// It will be utilized to update crypto stance by contact.
    UpdateCryptoStance(CryptoStance),
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CryptoStance {
    #[serde(rename = "supportive")]
    Supportive,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "against")]
    Against,
    #[default]
    NoStance,
}

impl std::fmt::Display for CryptoStance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoStance::Supportive => write!(f, "supportive"),
            CryptoStance::Against => write!(f, "against"),
            CryptoStance::Neutral => write!(f, "neutral"),
            CryptoStance::NoStance => write!(f, "no_stance"),
        }
    }
}

impl std::str::FromStr for CryptoStance {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "supportive" => Ok(CryptoStance::Supportive),
            "against" => Ok(CryptoStance::Against),
            "neutral" => Ok(CryptoStance::Neutral),
            "no_stance" => Ok(CryptoStance::NoStance),
            _ => Err(format!("Unknown crypto stance: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct AssemblyMember {
    pub id: String,
    pub r#type: String,
    pub code: String, // code could be duplicated by language

    pub created_at: u64,
    pub updated_at: u64,
    pub deleted_at: Option<u64>,

    pub name: String,
    pub party: String,
    pub district: District,
    // FIXME: consider update logic
    pub stance: Option<CryptoStance>,
    pub image_url: String,
    pub email: Option<String>,
    // pub email_verified: bool, // check email verified logic

    // Indexes, if deleted_at is set, all values of indexes must be empty.
    pub gsi1: String, // language
    // gsi2: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq)]
pub struct District {
    pub province: Option<String>, // None if it's a proportional representation
    pub district: String, 
}

impl TryFrom<(&str, &str)> for District {
    type Error = ServiceError;
    
    fn try_from((s, lang): (&str, &str)) -> Result<Self, Self::Error> {
        if s.trim().is_empty() {
            return Err(ServiceError::BadRequest);
        }

        fn create_district(province: Option<String>, district: String) -> District {
            District {
                province,
                district: district.trim().to_string(),
            }
        }

        if lang == "ko" {
            let parts: Vec<&str> = s.splitn(2, " ").collect();
            Ok(if parts.len() == 2 {
                create_district(Some(parts[0].to_string()), parts[1].to_string())
            } else {
                create_district(None, parts[0].to_string())
            })
        } else {
            let parts: Vec<&str> = if s.contains("((") {
                s.splitn(2, "((").collect()
            } else {
                s.splitn(2, "(").collect()
            };
            Ok(if parts.len() == 2 {
                let district = parts[1].trim_end_matches(')').to_string();
                if district.is_empty() {
                    create_district(None, parts[0].to_string())
                } else {
                    create_district(Some(parts[0].trim().to_string()), district)
                }
            } else {
                create_district(None, parts[0].to_string())
            })
        }
    }
}

impl std::fmt::Display for District {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.province {
            Some(province) => write!(f, "{} {}", province, self.district),
            None => write!(f, "{}", self.district),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ListAssemblyMembersRequest {
    pub size: Option<usize>,
    pub bookmark: Option<String>,
    pub lang: Option<Language>,
}

impl std::fmt::Display for ListAssemblyMembersRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let query = serde_urlencoded::to_string(&self).unwrap();

        write!(f, "{query}")
    }
}