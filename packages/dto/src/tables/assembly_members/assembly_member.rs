#![allow(unused)]
use std::str::FromStr;

use crate::*;
use bdk::prelude::*;
use by_types::QueryResponse;

use validator::ValidationError;

#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Default, Translate, ApiModel)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum CryptoStance {
    #[default]
    #[translate(en = "All Stance")]
    None = 99,
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

#[derive(
    Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, dioxus_translate::Translate, Copy,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum AssemblyMemberSorter {
    #[default]
    #[translate(ko = "이름순", en = "Sort by name")]
    Name = 1,
    #[translate(ko = "입장순", en = "Sort by stance")]
    Stance = 2,
    #[translate(ko = "정당순", en = "Sort by party")]
    Party = 3,
    #[translate(ko = "법안갯수순", en = "Sort by party")]
    Bills = 4,
}

#[derive(
    Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, dioxus_translate::Translate, Copy,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum SortOrder {
    #[default]
    #[translate(ko = "오름차순", en = "ascending")]
    Ascending = 1,
    #[translate(ko = "내림차순", en = "descending")]
    Descending = 2,
}

#[api_model(base = "/v1/assembly-members", table = assembly_members, iter_type = QueryResponse, queryable = [(sort = AssemblyMemberSorter, order = SortOrder)], action_by_id = [change_stance(code = String, stance = CryptoStance), send_verify_email])]
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
    #[api_model(summary, queryable)]
    pub party: String,
    #[api_model(summary)]
    pub district: String,

    #[api_model(summary)]
    pub en_name: String,
    #[api_model(summary)]
    pub en_party: String,
    #[api_model(summary)]
    pub en_district: Option<String>,

    #[api_model(summary, queryable, type = INTEGER, query_action = list_by_stance)]
    pub stance: CryptoStance,
    #[api_model(summary)]
    pub image_url: String,
    pub email: Option<String>,
    #[api_model(summary, many_to_many = proposers, foreign_table_name = bills, foreign_primary_key = bill_id, foreign_reference_key = member_id, aggregator = count)]
    pub no_of_bills: i64,
    #[api_model(many_to_many = proposers, foreign_table_name = bills, foreign_primary_key = bill_id, foreign_reference_key = member_id)]
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
