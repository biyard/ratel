//! "Request point conversion" button shown on the rewards page. Opens the
//! step-by-step `PointConversionModal` in a popup.

use crate::common::*;
use crate::features::launchpad_partner::LaunchpadPartnerTranslate;
use crate::features::launchpad_partner::views::PointConversionModal;

#[component]
pub fn PointConversionButton(available_points: i64) -> Element {
    let tr: LaunchpadPartnerTranslate = use_translate();
    let mut popup = use_popup();

    rsx! {
        Button {
            style: ButtonStyle::Primary,
            onclick: move |_| {
                popup
                    .open(rsx! {
                    PointConversionModal { available_points }
                })
                .with_title(tr.modal_title.to_string());
            },
            "{tr.convert_cta}"
        }
    }
}
