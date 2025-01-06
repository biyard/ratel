#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Member {
    #[serde(rename(deserialize = "HG_NM"))]
    name: String, // 한글 이름
    #[serde(rename(deserialize = "ENG_NM"))]
    eng_name: String, // 영문 이름
    #[serde(rename(deserialize = "POLY_NM"))]
    party: String, // 정당명
    #[serde(rename(deserialize = "ORIG_NM"))]
    district: String, // 선거구명
    #[serde(rename(deserialize = "MONA_CD"))]
    code: String, // 고유식별번호
}