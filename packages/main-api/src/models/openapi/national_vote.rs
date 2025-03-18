#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AssemblyVote {
    #[serde(rename(deserialize = "BILL_NM"))]
    pub bill_no: String,
    #[serde(rename(deserialize = "MONA_CD"))]
    pub member_code: String,
    #[serde(rename(deserialize = "RESULT_VOTE_MOD"))]
    pub result: i64,
}
