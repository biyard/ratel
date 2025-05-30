#![allow(non_snake_case)]
use crate::components::confirm_popup::ConfirmPopup;

use bdk::prelude::*;

#[component]
pub fn WelcomePopup(
    #[props(default ="welcome_popup".to_string())] id: String,
    lang: Language,
) -> Element {
    let tr = translate::<WelcomePopupTranslate>(&lang);

    rsx! {
        ConfirmPopup {
            lang,
            title: tr.title,
            description: tr.message,
            btn_label: tr.start,
        }
    }
}

translate! {
    WelcomePopupTranslate;

    title: {
        ko: "라텔에 오신 것을 환영합니다!",
        en: "Welcome to Ratel!",
    },

    message: {
        ko: "정책은 시민 참여에 의해 만들어집니다. 우리의 목소리가 정책 결정에 영향을 미칩니다. 라텔은 당신이 행동을 취하고 암호화폐 정책을 만들 수 있는 플랫폼을 제공합니다. 당신의 목소리는 중요합니다. 그것을 들려주고 암호화폐의 밝은 미래를 확보하는 데 도움을 주세요.",
        en: "Policy is shaped by civic engagement—when we speak up, policymakers listen. Ratel gives you a platform to take action and shape crypto policy. Your voice matters, so make it heard and help secure a bright future for crypto.",
    },

    start: {
        ko: "시작하기",
        en: "Start",
    },
}
