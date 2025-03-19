#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AssemblyProposer {
    // https://open.assembly.go.kr/portal/data/service/selectAPIServicePage.do/OK7XM1000938DS17215
    #[serde(rename(deserialize = "BILL_NO"))]
    pub bill_no: String,
    #[serde(rename(deserialize = "PUBL_PROPOSER"))]
    pub proposer_names: String, // pub
    #[serde(rename(deserialize = "RST_PROPOSER"))]
    pub representative_name: String,
    #[serde(rename(deserialize = "MEMBER_LIST"))]
    pub site_link: String,
}
