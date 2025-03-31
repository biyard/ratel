#![allow(non_snake_case)]
use bdk::prelude::*;

use crate::{components::socials::Socials, route::Route};

#[component]
pub fn Footer(lang: Language) -> Element {
    let tr: FooterTranslate = translate(&lang);

    rsx! {
        footer { class: "w-full bg-footer flex flex-row gap-10 items-center justify-center text-copyright font-normal text-sm/22 py-24 max-[900px]:!flex-col",
            span { {tr.copyright} }
            div { class: "flex gap-10",
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
}

#[component]
pub fn FooterWithSocial(lang: Language) -> Element {
    let tr: FooterTranslate = translate(&lang);

    rsx! {
        footer { class: "w-full bg-bg flex items-center justify-center",
            div { class: "w-full max-w-1177 flex flex-row gap-10 items-center justify-between text-copyright font-normal text-xs/22 py-24 max-tablet:!flex-col max-tablet:!py-20 max-tablet:!gap-20",
                div { class: "hidden max-tablet:!block",
                    Socials {
                        class: "flex flex-row items-center justify-center gap-30",
                        size: 28,
                    }
                }
                div { class: "h-full flex flex-row gap-10 items-center max-tablet:!flex-col max-tablet:!gap-0",
                    span { {tr.copyright} }
                    //desktop
                    div { class: "flex max-tablet:!hidden gap-10",
                        // FIXME: Link does not work with new_tab
                        // Link {
                        a {
                            class: "hover:text-white",
                            // to: Route::PrivacyPolicyPage { lang },
                            // new_tab: true,
                            href: "/{lang}/privacy-policy",
                            target: "_blank",
                            {tr.privacy}
                        }
                        // Link {
                        a {
                            class: "hover:text-white",
                            // to: Route::PrivacyPolicyPage { lang },
                            // new_tab: true,
                            href: "/{lang}/privacy-policy",
                            target: "_blank",
                            {tr.terms}
                        }
                    }


                    //mobile
                    div { class: "hidden max-tablet:!flex flex-row justify-center gap-10",
                        Link {
                            class: "hover:text-white",
                            to: Route::PrivacyPolicyPage { lang },
                            {tr.mobile_privacy}
                        }
                        p { "•" }
                        Link {
                            class: "hover:text-white",
                            to: Route::PrivacyPolicyPage { lang },
                            {tr.mobile_terms}
                        }
                    }
                }


                div { class: "block max-tablet:!hidden",
                    Socials {
                        class: "flex flex-row items-center justify-center gap-30",
                        size: 15,
                    }
                }
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
