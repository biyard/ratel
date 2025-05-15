#![allow(non_snake_case)]
use bdk::prelude::*;
use dioxus_popup::PopupService;

use crate::components::button::primary_button::PrimaryButton;

#[component]
pub fn LegalNoticePopup(lang: Language) -> Element {
    let mut p: PopupService = use_context();
    let tr: LegalNoticePopupTranslate = translate(&lang);

    use_effect(move || {
        p.with_title(tr.title);
    });

    rsx! {
        div { class: "w-full max-w-450 whitespace-pre-line flex flex-col gap-35",
            {tr.legal_full}
            PrimaryButton {
                width: "100%",
                onclick: move |_| {
                    p.close();
                },
                {tr.confirm}
            }
        }
    }
}

translate! {
    LegalNoticePopupTranslate;

    title: {
        ko: "초상권 및 법적 고지",
        en: "Portrait Rights & Legal Notice",
    },

    legal_full: {
        ko: "이 웹사이트는 국회의원에 대한 공개 정보를 정부 공식 소스를 기반으로 제공합니다. 정확성을 보장하기 위해 최선을 다하지만, 정보의 완전성, 신뢰성 또는 적시성을 보장하지 않습니다. 불일치 사항을 발견하면 신속한 검토 및 수정을 위해 연락 주시기 바랍니다.\n\n한국 정치인의 이미지는 공식 국회 웹사이트에서 직접 가져온 것이며 수정하지 않았습니다. 이 이미지는 적용 가능한 초상권 및 저작권 법률을 준수하여 사용되며 상업적 이득을 위해 사용되지 않습니다. 이 이미지는 정보 목적으로만 사용되며 상업적 이익을 추구하지 않습니다. 이 이미지의 소유권을 주장하지 않으며 어떠한 정치 단체나 개인도 지지하지 않습니다.\n\n이 웹사이트와 운영자는 제공된 정보의 사용으로 인한 모든 직접적 또는 간접적인 손해, 법적 주장 또는 분쟁에 대해 모든 책임을 부인합니다. 사용자는 모든 콘텐츠가 명시적 또는 묵시적으로 어떠한 종류의 보증도 없이 \"있는 그대로\" 및 \"사용 가능한\" 기반으로 제공된다는 것에 동의합니다. 이 웹사이트에 액세스함으로써 사용자는 이 콘텐츠 사용으로 인한 법적, 금융 또는 명예적 결과에 대해 책임이 없다는 것에 동의합니다.\n\n모든 우려 사항, 저작권 문의 또는 수정 요청에 대해서는 즉시 연락 주시기 바랍니다.",
        en: "This website provides publicly available information on members of the National Assembly based on official government sources. While we make every effort to ensure accuracy, we do not guarantee the completeness, reliability, or timeliness of the information. If you find any discrepancies, please contact us for prompt review and correction.\n\nImages of Korean politicians are sourced directly from the official National Assembly website without modification and are used in compliance with applicable portrait rights and copyright laws. These images are for informational purposes only and are not used for commercial gain. We do not claim ownership of these images, nor do we endorse any political entity or individual.\n\nThis website and its operators disclaim all liability for any direct or indirect damages, legal claims, or disputes arising from the use of the information provided. Users acknowledge that all content is provided on an \"as-is\" and \"as-available\" basis, without warranties of any kind, either express or implied. By accessing this website, you agree that we are not liable for any legal, financial, or reputational consequences resulting from the use of this content.\n\nFor any concerns, copyright inquiries, or modification requests, please contact us immediately.",
    }

    confirm: {
        ko: "확인",
        en: "Confirm",
    }
}
