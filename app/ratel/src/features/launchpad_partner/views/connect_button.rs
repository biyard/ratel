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
        button {
            r#type: "button",
            class: "inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-full text-sm font-extrabold cursor-pointer transition-transform hover:-translate-y-px",
            style: "background: linear-gradient(135deg,#ffd24a 0%,#fcb300 100%); color:#0a0a0a; box-shadow:0 10px 24px -10px rgba(252,179,0,0.5);",
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
