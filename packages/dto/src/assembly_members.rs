#![allow(unused)]
use crate::*;
use by_macros::*;
use by_types::QueryResponse;

#[cfg(feature = "server")]
use by_axum::aide;
use dioxus_translate::*;
use validator::ValidationError;

#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum AssemblyMemberAdminActionRequest {
    /// Fetches assembly members by Assembly Open API.
    /// And update the information of the assembly members.
    FetchMembers,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct AssemblyMemberResponse {
    pub request_id: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum AssemblyMemberByIdActionRequest {
    SendVerificationEmail {
        agree: bool,
    },
    UpdateCryptoStance {
        code: String,
        stance: CryptoStance,
        agree: bool,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum AssemblyMemberByIdActionResponse {
    SendVerificationEmail {
        email: String,
        request_code: String,
    },

    #[default]
    Ok,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum AssemblyMemberByIdAdminActionRequest {
    /// Manually, update crypto stance.
    /// It will be utilized to update crypto stance by contact.
    UpdateCryptoStance(CryptoStance),
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize, Translate, ApiModel)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum CryptoStance {
    #[default]
    NoStance = 0,
    Supportive = 1,
    Neutral = 2,
    Against = 3,
}

impl CryptoStance {
    pub fn iter() -> impl Iterator<Item = CryptoStance> {
        [
            CryptoStance::Supportive,
            CryptoStance::Neutral,
            CryptoStance::Against,
            CryptoStance::NoStance,
        ]
        .iter()
        .cloned()
    }
}

#[api_model(base = "/v1/assembly-members", table = assembly_members, iter_type = QueryResponse, action_by_id = [change_stance(code = String, stance = CryptoStance), send_verify_email])]
pub struct AssemblyMember {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, unique)]
    pub code: String,
    #[api_model(summary)]
    pub name: String,
    #[api_model(summary)]
    pub party: String,
    #[api_model(summary)]
    pub district: String,

    #[api_model(summary)]
    pub en_name: String,
    #[api_model(summary)]
    pub en_party: String,
    #[api_model(summary)]
    pub en_district: Option<String>,

    #[api_model(summary, type = INTEGER)]
    pub stance: CryptoStance,
    #[api_model(summary)]
    pub image_url: String,
    pub email: Option<String>,
    // pub email_verified: bool, // check email verified logic
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, ApiModel, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Lang {
    #[default]
    En = 1,
    Ko = 2,
}
