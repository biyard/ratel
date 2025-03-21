#![allow(non_snake_case)]
use bdk::prelude::*;

use crate::{
    pages::components::{Socials, subscription::MobileSubscription},
    route::Route,
};

#[component]
pub fn Footer(lang: Language) -> Element {
    rsx! {
        div { class: "hidden md:!block",
            DesktopFooter { lang }
        }
        div { class: "block md:!hidden",
            MobileFooter { lang }
        }
    }
}

#[component]
pub fn DesktopFooter(lang: Language) -> Element {
    let tr: FooterTranslate = translate(&lang);

    rsx! {
        footer { class: "w-screen bg-footer flex flex-row gap-10 items-center justify-center text-copyright font-normal text-sm/22 py-24",
            span { {tr.copyright} }
            Link {
                class: "hover:text-white",
                to: Route::PrivacyPolicyPage { lang },
                {tr.privacy}
            }
            Link {
                class: "hover:text-white",
                to: Route::PrivacyPolicyPage { lang },
                {tr.terms}
            }
        }
    }
}

#[component]
pub fn MobileFooter(lang: Language) -> Element {
    let tr: FooterTranslate = translate(&lang);

    rsx! {
        footer { class: "w-screen bg-footer flex flex-col items-center justify-center mt-[56px] text-copyright font-normal text-[14px] gap-[40px] px-[30px] pt-[20px] pb-[40px]",
            MobileSubscription { lang }
            Socials {
                class: "flex flex-row items-center justify-center gap-[50px]",
                size: 28,
            }
            div { class: "flex flex-col justify-center items-center",
                span { {tr.copyright} }
                div { class: "flex flex-row gap-[10px]",
                    Link {
                        class: "hover:text-white",
                        to: Route::PrivacyPolicyPage { lang },
                        {tr.mobile_privacy}
                    }
                    "•"
                    Link {
                        class: "hover:text-white",
                        to: Route::PrivacyPolicyPage { lang },
                        {tr.mobile_terms}
                    }
                }
            }
        }
    }
}

#[component]
pub fn FooterWithSocial(lang: Language) -> Element {
    let tr: FooterTranslate = translate(&lang);

    rsx! {
        footer { class: "w-full bg-bg flex flex-row gap-10 items-center justify-between text-copyright font-normal text-xs/22 py-24 h-50",
            div { class: "flex flex-row gap-10 items-center",
                span { {tr.copyright} }
                Link {
                    class: "hover:text-white",
                    to: Route::PrivacyPolicyPage { lang },
                    {tr.privacy}
                }
                Link {
                    class: "hover:text-white",
                    to: Route::PrivacyPolicyPage { lang },

                    {tr.terms}
                }
            }

            Socials {
                class: "flex flex-row items-center justify-center gap-30",
                size: 15,
            }
        }
    }
}

translate! {
    FooterTranslate;

    copyright: {
        ko: "© 2025 Ratel Foundation.",
        en: "© 2025 Ratel Foundation.",
    }

    privacy: {
        ko: "• 개인 정보 보호 정책",
        en: "• Privacy",
    },

    terms: {
        ko: "• 서비스 약관",
        en: "• Terms",
    },

    mobile_privacy: {
        ko: "개인 정보 보호 정책",
        en: "Privacy Policy",
    },

    mobile_terms: {
        ko: "서비스 약관",
        en: "Terms of Service",
    },

    sitemap: {
        ko: "• 사이트맵",
        en: "• Sitemap",
    },
}
