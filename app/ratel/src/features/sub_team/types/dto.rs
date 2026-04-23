use crate::common::*;

use crate::features::sub_team::models::SubTeamFormFieldType;

// ── Settings ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamSettingsResponse {
    pub is_parent_eligible: bool,
    pub min_sub_team_members: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UpdateSubTeamSettingsRequest {
    #[serde(default)]
    pub is_parent_eligible: Option<bool>,
    #[serde(default)]
    pub min_sub_team_members: Option<i32>,
}

// ── Form fields ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamFormFieldResponse {
    pub id: String,
    pub label: String,
    pub field_type: SubTeamFormFieldType,
    pub required: bool,
    pub order: i32,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreateSubTeamFormFieldRequest {
    pub label: String,
    pub field_type: SubTeamFormFieldType,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub order: Option<i32>,
    #[serde(default)]
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UpdateSubTeamFormFieldRequest {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub field_type: Option<SubTeamFormFieldType>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub order: Option<i32>,
    #[serde(default)]
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ReorderFormFieldsRequest {
    pub field_ids: Vec<String>,
}

// ── Documents ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubTeamDocumentResponse {
    pub id: String,
    pub title: String,
    pub body: String,
    pub required: bool,
    pub order: i32,
    pub body_hash: String,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreateSubTeamDocumentRequest {
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UpdateSubTeamDocumentRequest {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub required: Option<bool>,
    #[serde(default)]
    pub order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ReorderDocumentsRequest {
    pub doc_ids: Vec<String>,
}

// ── Public apply context ────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ApplyContextDocument {
    pub id: String,
    pub title: String,
    pub body: String,
    pub body_hash: String,
    pub order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ApplyContextResponse {
    pub is_parent_eligible: bool,
    pub min_sub_team_members: i32,
    pub recognized_count: i64,
    pub pending_count: i64,
    pub form_fields: Vec<SubTeamFormFieldResponse>,
    pub required_docs: Vec<ApplyContextDocument>,
}

// ── Conversions from models ─────────────────────────────────────

#[cfg(feature = "server")]
impl From<crate::features::sub_team::models::SubTeamFormField> for SubTeamFormFieldResponse {
    fn from(f: crate::features::sub_team::models::SubTeamFormField) -> Self {
        let id = match &f.sk {
            EntityType::SubTeamFormField(id) => id.clone(),
            _ => String::new(),
        };
        Self {
            id,
            label: f.label,
            field_type: f.field_type,
            required: f.required,
            order: f.order,
            options: f.options,
        }
    }
}

#[cfg(feature = "server")]
impl From<crate::features::sub_team::models::SubTeamDocument> for SubTeamDocumentResponse {
    fn from(d: crate::features::sub_team::models::SubTeamDocument) -> Self {
        let id = match &d.sk {
            EntityType::SubTeamDocument(id) => id.clone(),
            _ => String::new(),
        };
        Self {
            id,
            title: d.title,
            body: d.body,
            required: d.required,
            order: d.order,
            body_hash: d.body_hash,
            updated_at: d.updated_at,
        }
    }
}

#[cfg(feature = "server")]
impl From<crate::features::sub_team::models::SubTeamDocument> for ApplyContextDocument {
    fn from(d: crate::features::sub_team::models::SubTeamDocument) -> Self {
        let id = match &d.sk {
            EntityType::SubTeamDocument(id) => id.clone(),
            _ => String::new(),
        };
        Self {
            id,
            title: d.title,
            body: d.body,
            body_hash: d.body_hash,
            order: d.order,
        }
    }
}
