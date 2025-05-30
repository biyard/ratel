use crate::{
    components::{
        button::{ButtonSize, primary_button::PrimaryLink},
        confirm_popup::WelcomeHeader,
    },
    route::Route,
};
use bdk::prelude::*;
use dioxus_popup::PopupService;

#[component]
pub fn SponsorConfirmPopup(
    #[props(default ="welcome_popup".to_string())] id: String,
    lang: Language,
) -> Element {
    let tr = translate::<SponsorPopupTranslate>(&lang);
    let mut popup: PopupService = use_context();
    rsx! {
        div { id, class: "max-w-400 w-full mx-5 max-mobile:!max-w-full",
            div { class: "w-full flex flex-col gap-35",
                WelcomeHeader { lang, title: tr.title, description: tr.message }

                PrimaryLink {
                    size: ButtonSize::Normal,
                    to: Route::LandingPage {},
                    onclick: move |_| {
                        popup.close();
                    },
                    {tr.start}
                }
            }
        }
    }
}

translate! {
    SponsorPopupTranslate;

    title: {
        ko: "지지해주셔서 감사합니다!",
        en: "Thank you for your support!",
    },

    message: {
        ko: "정책은 시민 참여에 의해 만들어집니다. 우리의 목소리가 정책 결정에 영향을 미칩니다. 라텔은 당신이 행동을 취하고 암호화폐 정책을 만들 수 있는 플랫폼을 제공합니다. 당신의 목소리는 중요합니다. 그것을 들려주고 암호화폐의 밝은 미래를 확보하는 데 도움을 주세요.",
        en: "Policy is shaped by civic engagement—when we speak up, policymakers listen. Ratel gives you a platform to take action and shape crypto policy. Your voice matters, so make it heard and help secure a bright future for crypto.",
    },

    start: {
        ko: "홈으로 가기",
        en: "Go to home",
    },
}
