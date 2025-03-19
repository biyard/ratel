#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Member {
    // https://open.assembly.go.kr/portal/data/service/selectAPIServicePage.do/OWSSC6001134T516707
    #[serde(rename(deserialize = "HG_NM"))]
    pub name: String, // Korean Name
    #[serde(rename(deserialize = "POLY_NM"))]
    pub party: String, // Korean Party Name
    #[serde(rename(deserialize = "ORIG_NM"))]
    pub district: String, // Korean District Name
    #[serde(rename(deserialize = "MONA_CD"))]
    pub code: String, // Unique Code
    #[serde(rename(deserialize = "E_MAIL"))]
    pub email: Option<String>, // Email
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EnMember {
    // https://open.assembly.go.kr/portal/data/service/selectAPIServicePage.do/OOWY4R001216HX11447
    #[serde(rename(deserialize = "NAAS_EN_NM"))]
    pub name: String, // English Name
    #[serde(rename(deserialize = "PLPT_NM"))]
    pub party: String, // English Party Name
    #[serde(rename(deserialize = "ELECD_NM"))]
    pub district: Option<String>, // English District Name
    #[serde(rename(deserialize = "NAAS_EMAIL_ADDR"))]
    pub email: Option<String>, // Email
}
