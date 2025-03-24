#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AssemblyBill {
    // https://open.assembly.go.kr/portal/data/service/selectAPIServicePage.do/OOWY4R001216HX11462
    #[serde(rename(deserialize = "BILL_NO"))]
    pub bill_no: String,
    #[serde(rename(deserialize = "BILL_NM"))]
    pub title: String,
    #[serde(rename(deserialize = "LINK_URL"))]
    pub site_link: String, // Link to bill summary site (include hwp, pdf link)
    #[serde(rename(deserialize = "BILL_ID"))]
    pub bill_id: String, // need get vote result
    #[serde(rename(deserialize = "PPSL_DT"))]
    pub date: String, // ex. 2009-06-30
}
