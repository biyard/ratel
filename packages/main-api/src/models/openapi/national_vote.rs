#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AssemblyVote {
    // https://open.assembly.go.kr/portal/data/service/selectAPIServicePage.do/OPR1MQ000998LC12535
    #[serde(rename(deserialize = "BILL_NM"))]
    pub bill_no: String,
    #[serde(rename(deserialize = "MONA_CD"))]
    pub member_code: String,
    #[serde(rename(deserialize = "RESULT_VOTE_MOD"))]
    pub result: i64,
}
