use bdk::prelude::*;

#[api_model(base = "/m1/us/bills", table = us_bills, action = [fetch_bills(start_bill_no = i64, end_bill_no = i64)])]
pub struct USBillWriter {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = [fetch_bill, fetch_bills, fetch_recent_bills])]
    pub congress: i64, // current congress number is 118
    #[api_model(summary, type = INTEGER, action = [fetch_bill, fetch_bills, fetch_recent_bills])]
    pub bill_type: USBillType, // e.g., hr, s, hjres, sjres, hconres, sconres, hres, or sres
    #[api_model(summary, action = [fetch_bill, fetch_recent_bills])]
    pub bill_no: i64, // e.g., 3076

    #[api_model(summary)]
    pub title: String,
    #[api_model(summary)]
    pub summary: String,

    #[api_model(summary, unique)]
    pub bill_id: String, // e.g., hr3076-118

    #[api_model(summary)]
    pub html_url: Option<String>,
    #[api_model(summary)]
    pub pdf_url: Option<String>,
    #[api_model(summary)]
    pub xml_url: Option<String>,

    #[api_model(summary, type = INTEGER)]
    pub origin_chamber: Chamber, // "House" or "Senate"

    #[api_model(summary)]
    pub action_date: String, // latest action date

    #[api_model(summary)]
    pub update_date: String, // bill update date

    #[api_model(summary, version = v0.1, type = INTEGER)]
    pub industry: PolicyArea,
    // FIXME: Spec provided by api is not clear
    // #[api_model(summary, version = v0.1, type = INTEGER)]
    // pub status: Option<USBillStatus>,
}

// https://www.govinfo.gov/help/bills
// https://www.house.gov/the-house-explained/the-legislative-process/bills-resolutions
#[derive(Debug, Clone, Eq, PartialEq, ApiModel, Default, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum USBillType {
    #[translate(en = "Unknown", ko = "알 수 없음")]
    #[default]
    Unknown = 0,

    // hr
    #[translate(code = "hr", en = "House Bill", ko = "하원 법안")]
    HouseBill = 1,

    // s
    #[translate(code = "s", en = "Senate Bill", ko = "상원 법안")]
    SenateBill = 2,

    // hjres
    #[translate(code = "hjres", en = "House Joint Resolution", ko = "하원 공동 결의안")]
    HouseJointResolution = 3,

    // sjres
    #[translate(
        code = "sjres",
        en = "Senate Joint Resolution",
        ko = "상원 공동 결의안"
    )]
    SenateJointResolution = 4,

    // hconres
    #[translate(
        code = "hconres",
        en = "House Concurrent Resolution",
        ko = "하원 동시 결의안"
    )]
    HouseConcurrentResolution = 5,

    // sconres
    #[translate(
        code = "sconres",
        en = "Senate Concurrent Resolution",
        ko = "상원 동시 결의안"
    )]
    SenateConcurrentResolution = 6,

    // hres
    #[translate(code = "hres", en = "House Simple Resolution", ko = "하원 단순 결의안")]
    HouseSimpleResolution = 7,

    // sres
    #[translate(
        code = "sres",
        en = "Senate Simple Resolution",
        ko = "상원 단순 결의안"
    )]
    SenateSimpleResolution = 8,
}

impl USBillType {
    pub fn to_code(&self) -> &'static str {
        match self {
            USBillType::Unknown => "unknown",
            USBillType::HouseBill => "hr",
            USBillType::SenateBill => "s",
            USBillType::HouseJointResolution => "hjres",
            USBillType::SenateJointResolution => "sjres",
            USBillType::HouseConcurrentResolution => "hconres",
            USBillType::SenateConcurrentResolution => "sconres",
            USBillType::HouseSimpleResolution => "hres",
            USBillType::SenateSimpleResolution => "sres",
        }
    }

    pub fn from_i64(value: i64) -> Option<Self> {
        match value {
            0 => Some(USBillType::Unknown),
            1 => Some(USBillType::HouseBill),
            2 => Some(USBillType::SenateBill),
            3 => Some(USBillType::HouseJointResolution),
            4 => Some(USBillType::SenateJointResolution),
            5 => Some(USBillType::HouseConcurrentResolution),
            6 => Some(USBillType::SenateConcurrentResolution),
            7 => Some(USBillType::HouseSimpleResolution),
            8 => Some(USBillType::SenateSimpleResolution),
            _ => None,
        }
    }
}

// https://www.congress.gov/search?q=%7B%22source%22%3A%22legislation%22%2C%22bill-status%22%3A%22committee%22%7D
#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum USBillStatus {
    #[translate(en = "Unknown", ko = "알 수 없음")]
    #[default]
    Unknown = 0,

    #[translate(en = "Introduced", ko = "발의")]
    Introduced = 1,

    #[translate(en = "Committee Consideration", ko = "위원회 심사")]
    CommitteeConsideration = 2,

    #[translate(en = "Floor Consideration", ko = "상원/하원 심사")]
    FloorConsideration = 3,

    #[translate(en = "Failed in One Chamber", ko = "상원/하원 심사 실패")]
    FailedOneChamber = 4,

    #[translate(en = "Passed in One Chamber", ko = "상원/하원 심사 통과")]
    PassedOneChamber = 5,

    #[translate(en = "Passed in Both Chambers", ko = "상하원 모두 심사 통과")]
    PassedBothChambers = 6,

    #[translate(en = "Resolving Differences", ko = "상하원 조정")]
    ResolvingDifferences = 7,

    #[translate(en = "To President", ko = "대통령에게 송부")]
    ToPresident = 8,

    #[translate(en = "Veto Actions", ko = "대통령 거부")]
    VetoActions = 9,

    #[translate(en = "Became Law", ko = "법률 제정")]
    BecameLaw = 10,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Chamber {
    #[translate(en = "Unknown", ko = "알 수 없음")]
    #[default]
    Unknown = 0,
    #[translate(en = "House", ko = "하원")]
    House = 1,
    #[translate(en = "Senate", ko = "상원")]
    Senate = 2,
}

// https://www.congress.gov/browse/policyarea
#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum PolicyArea {
    #[default]
    Others = 0,

    #[translate(en = "Agriculture and Food", ko = "농업 및 식품")]
    AgricultureAndFood = 1,
    #[translate(en = "Animals", ko = "동물")]
    Animals = 2,
    #[translate(en = "Armed Forces and National Security", ko = "군대 및 국가 안보")]
    ArmedForcesAndNationalSecurity = 3,
    #[translate(en = "Arts, Culture, Religion", ko = "예술, 문화, 종교")]
    ArtsCultureReligion = 4,
    #[translate(
        en = "Civil Rights and Liberties, Minority Issues",
        ko = "시민권 및 자유, 소수자 문제"
    )]
    CivilRightsAndLiberties = 5,
    #[translate(en = "Commerce", ko = "상업")]
    Commerce = 6,
    #[translate(en = "Congress", ko = "의회")]
    Congress = 7,
    #[translate(en = "Crime and Law Enforcement", ko = "범죄 및 법 집행")]
    CrimeAndLawEnforcement = 8,
    #[translate(en = "Economics and Public Finance", ko = "경제학 및 공공 재정")]
    EconomicsAndPublicFinance = 9,
    #[translate(en = "Education", ko = "교육")]
    Education = 10,
    #[translate(en = "Emergency Management", ko = "비상 관리")]
    EmergencyManagement = 11,
    #[translate(en = "Energy", ko = "에너지")]
    Energy = 12,
    #[translate(en = "Environmental Protection", ko = "환경 보호")]
    EnvironmentalProtection = 13,
    #[translate(en = "Families", ko = "가족")]
    Families = 14,
    #[translate(en = "Finance and Financial Sector", ko = "금융 및 금융 부문")]
    FinanceAndFinancialSector = 15,
    #[translate(
        en = "Foreign Trade and International Finance",
        ko = "외국 무역 및 국제 금융"
    )]
    ForeignTradeAndInternationalFinance = 16,
    #[translate(en = "Government Operations and Politics", ko = "정부 운영 및 정치")]
    GovernmentOperationsAndPolitics = 17,
    #[translate(en = "Health", ko = "건강")]
    Health = 18,
    #[translate(
        en = "Housing and Community Development",
        ko = "주택 및 지역 사회 개발"
    )]
    HousingAndCommunityDevelopment = 19,
    #[translate(en = "Immigration", ko = "이민")]
    Immigration = 20,
    #[translate(en = "International Affairs", ko = "국제 문제")]
    InternationalAffairs = 21,
    #[translate(en = "Labor and Employment", ko = "노동 및 고용")]
    LaborAndEmployment = 22,
    #[translate(en = "Law", ko = "법")]
    Law = 23,
    #[translate(en = "Native Americans", ko = "아메리카 원주민")]
    NativeAmericans = 24,
    #[translate(
        en = "Public Lands and Natural Resources",
        ko = "공공 토지 및 천연 자원"
    )]
    PublicLandsAndNaturalResources = 25,
    #[translate(en = "Science, Technology, Communications", ko = "과학, 기술, 통신")]
    ScienceTechnologyCommunications = 26,
    #[translate(en = "Social Welfare", ko = "사회 복지")]
    SocialWelfare = 27,
    #[translate(en = "Sports and Recreation", ko = "스포츠 및 레크리에이션")]
    SportsAndRecreation = 28,
    #[translate(en = "Taxation", ko = "세금")]
    Taxation = 29,
    #[translate(en = "Transportation and Public Works", ko = "교통 및 공공 사업")]
    TransportationAndPublicWorks = 30,
    #[translate(en = "Water Resources Development", ko = "수자원 개발")]
    WaterResourcesDevelopment = 31,

    #[translate(en = "Crypto", ko = "암호화폐")]
    Crypto = 32,
}
