mod bill_details;
mod bill_info;
mod bill_subject;
mod bill_summary;
mod bill_text;
mod bill_titles;
mod common;

pub use bill_details::*;
pub use bill_info::*;
pub use bill_subject::*;
pub use bill_summary::*;
pub use bill_text::*;
pub use bill_titles::*;
pub use common::*;

use dto::USBillWriter;

pub fn convert_to_bill_writer(
    detail: BillDetail,
    titles: BillTitles,
    subject: BillSubject,
    summary: BillSummaries,
    texts: BillTexts,
) -> USBillWriter {
    USBillWriter {
        congress: detail.bill.congress,
        bill_no: detail.bill.number.parse().unwrap_or_default(),
        bill_type: detail.convert_bill_type(),
        title: titles.get_display_title().unwrap_or_default(),
        summary: summary.get_last_texts(),

        bill_id: format!(
            "{}{}-{}",
            detail.bill.congress,
            detail.convert_bill_type(),
            detail.bill.number
        ),

        html_url: texts.get_html_url(),
        pdf_url: texts.get_pdf_url(),
        xml_url: texts.get_xml_url(),

        origin_chamber: detail.get_origin_chamber(),
        action_date: detail.bill.latest_action.action_date,
        update_date: detail.bill.update_date,
        industry: subject.into_policy_area(),
        ..Default::default()
    }
}
