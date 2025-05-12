use super::common::PolicyArea;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillSubject {
    pub subjects: BillSubjectItem,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillSubjectItem {
    /// 1: 정책 영역
    pub policy_area: PolicyArea,

    /// 2: 법안 주제
    #[serde(rename = "legislativeSubjects")]
    pub legislative_subjects: Vec<LegislativeSubject>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegislativeSubject {
    /// 1: 법안 주제 이름
    pub name: String,

    /// 2: 업데이트 날짜
    #[serde(rename = "updateDate")]
    pub update_date: String,
}

impl BillSubject {
    pub fn into_policy_area(&self) -> dto::PolicyArea {
        convert_policy_area(&self.subjects.policy_area.name)
    }
}

fn convert_policy_area(r#type: &str) -> dto::PolicyArea {
    match r#type {
        "Agriculture and Food" => dto::PolicyArea::AgricultureAndFood,
        "Animals" => dto::PolicyArea::Animals,
        "Armed Forces and National Security" => dto::PolicyArea::ArmedForcesAndNationalSecurity,
        "Arts, Culture, Religion" => dto::PolicyArea::ArtsCultureReligion,
        "Civil Rights and Liberties, Minority Issues" => dto::PolicyArea::CivilRightsAndLiberties,
        "Commerce" => dto::PolicyArea::Commerce,
        "Congress" => dto::PolicyArea::Congress,
        "Crime and Law Enforcement" => dto::PolicyArea::CrimeAndLawEnforcement,
        "Economics and Public Finance" => dto::PolicyArea::EconomicsAndPublicFinance,
        "Education" => dto::PolicyArea::Education,
        "Emergency Management" => dto::PolicyArea::EmergencyManagement,
        "Energy" => dto::PolicyArea::Energy,
        "Environmental Protection" => dto::PolicyArea::EnvironmentalProtection,
        "Families" => dto::PolicyArea::Families,
        "Finance and Financial Sector" => dto::PolicyArea::FinanceAndFinancialSector,
        "Foreign Trade and International Finance" => {
            dto::PolicyArea::ForeignTradeAndInternationalFinance
        }
        "Government Operations and Politics" => dto::PolicyArea::GovernmentOperationsAndPolitics,
        "Health" => dto::PolicyArea::Health,
        "Housing and Community Development" => dto::PolicyArea::HousingAndCommunityDevelopment,
        "Immigration" => dto::PolicyArea::Immigration,
        "International Affairs" => dto::PolicyArea::InternationalAffairs,
        "Labor and Employment" => dto::PolicyArea::LaborAndEmployment,
        "Law" => dto::PolicyArea::Law,
        "Native Americans" => dto::PolicyArea::NativeAmericans,
        "Public Lands and Natural Resources" => dto::PolicyArea::PublicLandsAndNaturalResources,
        "Science, Technology, Communications" => dto::PolicyArea::ScienceTechnologyCommunications,
        "Social Welfare" => dto::PolicyArea::SocialWelfare,
        "Sports and Recreation" => dto::PolicyArea::SportsAndRecreation,
        "Taxation" => dto::PolicyArea::Taxation,
        "Transportation and Public Works" => dto::PolicyArea::TransportationAndPublicWorks,
        "Water Resources Development" => dto::PolicyArea::WaterResourcesDevelopment,
        _ => dto::PolicyArea::Others,
    }
}
