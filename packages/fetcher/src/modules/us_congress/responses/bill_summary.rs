use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillSummaries {
    summaries: Vec<BillSummaryItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BillSummaryItem {
    // 1. Action date
    #[serde(rename = "actionDate")]
    pub action_date: String,
    // 2. Action description
    #[serde(rename = "actionDesc")]
    pub action_desc: String,
    // 3. Summary text
    pub text: String,
    // 4. Update date
    #[serde(rename = "updateDate")]
    pub update_date: String,
    // 5. Version code
    #[serde(rename = "versionCode")]
    pub version_code: String,
}

impl BillSummaries {
    pub fn extract_summary_texts(&self) -> Vec<String> {
        self.summaries
            .iter()
            .map(|item| item.text.clone())
            .collect()
    }

    pub fn get_last_texts(&self) -> String {
        let summary = self.extract_summary_texts();

        match summary.last() {
            Some(last_summary) => last_summary.to_string(),
            None => "".to_string(),
        }
    }

    pub fn convert_bill_status(&self) -> dto::USBillStatus {
        let summary = self.extract_summary_texts();

        let _last_summary = match summary.last() {
            Some(last_summary) => last_summary,
            None => return dto::USBillStatus::Unknown,
        };

        // let action_text = last_summary.action_desc.to_lowercase();

        // FIXME: Spec provided by api is not clear
        // if action_text.contains("public law") {
        //     dto::USBillStatus::BecameLaw
        // } else if action_text.contains("failed") || action_text.contains("rejected") {
        //     dto::USBillStatus::FailedOneChamber
        // } else if action_text.contains("passed")
        //     && (action_text.contains("house") && action_text.contains("senate"))
        // {
        //     dto::USBillStatus::PassedBothChambers
        // } else if action_text.contains("passed") {
        //     dto::USBillStatus::PassedOneChamber
        // } else if action_text.contains("reported") || action_text.contains("committee") {
        //     dto::USBillStatus::ReportedByCommittee
        // } else if action_text.contains("introduced") {
        //     dto::USBillStatus::Introduced
        // } else if action_text.contains("signed") || action_text.contains("president") {
        //     dto::USBillStatus::ToPresident
        // } else if action_text.contains("vetoed") {
        //     dto::USBillStatus::Vetoed
        // } else if action_text.contains("floor") {
        //     dto::USBillStatus::FloorConsideration
        // } else {
        //     dto::USBillStatus::Unknown
        // }
        dto::USBillStatus::Unknown
    }
}
