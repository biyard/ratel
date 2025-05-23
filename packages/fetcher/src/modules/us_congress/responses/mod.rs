mod bill_details;
mod bill_info;
mod bill_summary;
mod bill_text;
mod bill_titles;
mod common;

pub use bill_details::*;
pub use bill_info::*;
pub use bill_summary::*;
pub use bill_text::*;
pub use bill_titles::*;

use dto::USBillWriter;

pub fn convert_to_bill_writer(
    detail: BillDetail,
    summary: BillSummaries,
    texts: BillTexts,
) -> USBillWriter {
    USBillWriter {
        congress: detail.congress,
        bill_no: detail.number.parse().unwrap_or_default(),
        bill_type: detail.convert_bill_type(),
        title: detail.title.clone(),
        summary: summary.get_last_texts(),

        bill_id: format!(
            "{}{}-{}",
            detail.congress,
            detail.convert_bill_type(),
            detail.number
        ),

        html_url: texts.get_html_url(),
        pdf_url: texts.get_pdf_url(),
        xml_url: texts.get_xml_url(),

        origin_chamber: detail.get_origin_chamber(),
        industry: detail.get_policy_area(),
        action_date: detail.latest_action.action_date,
        update_date: detail.update_date,
        ..Default::default()
    }
}
