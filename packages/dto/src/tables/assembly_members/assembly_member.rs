#![allow(unused)]
use std::str::FromStr;

use crate::*;
use bdk::prelude::*;
use by_types::QueryResponse;

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

#[derive(Debug, Clone, Eq, PartialEq, Default, Translate, ApiModel)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum CryptoStance {
    #[default]
    #[translate(en = "No Stance")]
    NoStance = 0,
    #[translate(en = "Pro-Crypto")]
    ProCrypto = 1,
    #[translate(en = "Neutral")]
    Neutral = 2,
    #[translate(en = "Anti-Crypto")]
    AntiCrypto = 3,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Translate, ApiModel)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum Party {
    #[default]
    #[translate(en = "Party")]
    None = 0,
    #[translate(en = "DP", ko = "더불어민주당")]
    DemocraticParty = 1,
    #[translate(en = "PPP", ko = "국민의힘")]
    PeoplePowerParty = 2,
    #[translate(en = "RKP", ko = "조국혁신당")]
    RebuildingKoreaParty = 3,
    #[translate(en = "Jinbo", ko = "진보당")]
    JinboParty = 4,
    #[translate(en = "Reform", ko = "개혁신당")]
    ReformParty = 5,
    #[translate(en = "Basic Income", ko = "기본소득당")]
    BasicIncomeParty = 6,
    #[translate(en = "SDP", ko = "사회민주당")]
    SocialDemocraticParty = 7,
    #[translate(en = "", ko = "무소속")]
    Independent = 8,
}

// TODO(api): implement list_by_stance
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

    #[api_model(summary, type = INTEGER, query_action = list_by_stance)]
    pub stance: CryptoStance,
    #[api_model(summary)]
    pub image_url: String,
    pub email: Option<String>,
    // pub email_verified: bool, // check email verified logic
    #[api_model(many_to_many = proposers, foreign_table_name = bills, foreign_primary_key = bill_id, foreign_reference_key = member_id, unique)]
    #[serde(default)]
    pub bills: Vec<Bill>,
}

impl AssemblyMemberSummary {
    pub fn stance_color(&self) -> &'static str {
        match self.stance {
            CryptoStance::ProCrypto => "bg-c-c-20",
            CryptoStance::Neutral => "bg-c-pp-20",
            CryptoStance::AntiCrypto => "bg-c-p-20",
            _ => "bg-c-wg-50",
        }
    }

    pub fn name(&self, lang: &Language) -> &str {
        match lang {
            Language::En => &self.en_name,
            _ => &self.name,
        }
    }

    pub fn party_enum(&self) -> Party {
        Party::from_str(&self.party).unwrap_or_default()
    }

    pub fn party(&self, lang: &Language) -> &str {
        Party::from_str(&self.party)
            .unwrap_or_default()
            .translate(lang)
    }
}
