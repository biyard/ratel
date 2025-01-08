#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Member {
    #[serde(rename(deserialize = "HG_NM"))]
    pub name: String, // 한글 이름
    #[serde(rename(deserialize = "POLY_NM"))]
    pub party: String, // 정당명
    #[serde(rename(deserialize = "ORIG_NM"))]
    pub district: String, // 선거구명
    #[serde(rename(deserialize = "MONA_CD"))]
    pub code: String, // 고유식별번호
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EnMember {
    #[serde(rename(deserialize = "NAAS_EN_NM"))]
    pub name: String, // 영문 이름
    #[serde(rename(deserialize = "PLPT_NM"))]
    pub party: String, // 영문 정당명
    #[serde(rename(deserialize = "ELECD_NM"))]
    pub district: String, // 영문 선거구명
}