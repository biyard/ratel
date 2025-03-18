#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AssemblyBill {
    #[serde(rename(deserialize = "BILL_NO"))]
    pub bill_no: String,
    #[serde(rename(deserialize = "BILL_NM"))]
    pub title: String,
    #[serde(rename(deserialize = "LINK_URL"))]
    pub site_link: String, // Link to bill summary site (include hwp, pdf link)
    #[serde(rename(deserialize = "BILL_ID"))]
    pub bill_id: String, // need get vote result
}
