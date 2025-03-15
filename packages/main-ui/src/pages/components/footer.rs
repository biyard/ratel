#![allow(non_snake_case)]
use bdk::prelude::*;

#[component]
pub fn Footer(lang: Language) -> Element {
    let tr: FooterTranslate = translate(&lang);

    rsx! {
        footer { class: "w-full bg-footer flex flex-row gap-10 items-center justify-center text-copyright font-normal text-sm/22 py-24",
            span { {tr.copyright} }
            a { class: "hover:text-white", href: "/privacy", {tr.privacy} }
            a { class: "hover:text-white", href: "/terms", {tr.terms} }
            a { class: "hover:text-white", href: "/sitemap", {tr.sitemap} }
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

    sitemap: {
        ko: "• 사이트맵",
        en: "• Sitemap",
    },
}
