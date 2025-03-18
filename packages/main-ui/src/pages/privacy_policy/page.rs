#![allow(non_snake_case)]
use crate::pages::components::Footer;

use super::*;
use bdk::prelude::*;
use i18n::*;

#[component]
pub fn PrivacyPolicyPage(lang: Language) -> Element {
    let tr: PrivacyPolicyTranslate = translate(&lang);

    rsx! {
        by_components::meta::MetaPage { title: tr.title, description: tr.description }

        div { class: "w-full min-h-screen flex flex-col items-center justify-between gap-50",
            div {
                id: "privacy-policy",
                class: "w-full max-w-1177 mt-160 flex flex-col items-start justify-start gap-30 px-10",
                div { class: "flex flex-col gap-8 items-start",
                    h1 { class: "text-[32px]/22.5 font-bold text-text-primary", {tr.title} }
                    p { class: "text-[15px]/22.5 font-normal text-text-secondary",
                        {tr.date}
                    }
                }

                p { class: "text-[15px]/22.5 font-normal text-text-secondary", {tr.description} }

                div { class: "flex flex-col gap-8 items-start",
                    h2 { class: "text-2xl/22.5 font-bold text-text-primary", {tr.t1} }
                    p { class: "text-[15px]/22.5 font-normal text-text-secondary",
                        {tr.t1_d1}
                    }
                }

                div { class: "flex flex-col gap-8 items-start",
                    h2 { class: "text-2xl/22.5 font-bold text-text-primary", {tr.t2} }
                    p { class: "text-[15px]/22.5 font-normal text-text-secondary",
                        {tr.t2_d1}
                    }
                    ul { class: "list-disc ml-30 pl-5",
                        li { class: "text-[15px]/22.5 font-normal text-text-secondary",
                            {tr.t2_d2}
                        }
                        li { class: "text-[15px]/22.5 font-normal text-text-secondary",
                            {tr.t2_d3}
                        }
                        li { class: "text-[15px]/22.5 font-normal text-text-secondary",
                            {tr.t2_d4}
                        }
                        li { class: "text-[15px]/22.5 font-normal text-text-secondary",
                            {tr.t2_d5}
                        }

                    }
                    p { class: "text-[15px]/22.5 font-normal text-text-secondary",
                        {tr.t2_d6}
                    }
                }


                div { class: "flex flex-col gap-8 items-start",
                    h2 { class: "text-2xl/22.5 font-bold text-text-primary", {tr.t3} }
                    p { class: "text-[15px]/22.5 font-normal text-text-secondary",
                        {tr.t3_d1}
                    }
                }

                div { class: "flex flex-col gap-8 items-start",
                    h2 { class: "text-2xl/22.5 font-bold text-text-primary", {tr.t4} }
                    p { class: "text-[15px]/22.5 font-normal text-text-secondary",
                        {tr.t4_d1}
                    }
                    ul { class: "list-disc ml-30 pl-5",
                        li { class: "text-[15px]/22.5 font-normal text-text-secondary",
                            {tr.t4_d2}
                        }
                        li { class: "text-[15px]/22.5 font-normal text-text-secondary",
                            {tr.t4_d3}
                        }
                        li { class: "text-[15px]/22.5 font-normal text-text-secondary",
                            {tr.t4_d4}
                        }
                    }
                    p { class: "text-[15px]/22.5 font-normal text-text-secondary",
                        {tr.t2_d5}
                    }
                }


                div { class: "flex flex-col gap-8 items-start",
                    h2 { class: "text-2xl/22.5 font-bold text-text-primary", {tr.t5} }
                    p { class: "text-[15px]/22.5 font-normal text-text-secondary",
                        {tr.t5_d1}
                    }
                }

                div { class: "flex flex-col gap-8 items-start",
                    h2 { class: "text-2xl/22.5 font-bold text-text-primary", {tr.t6} }
                    p { class: "text-[15px]/22.5 font-normal text-text-secondary",
                        {tr.t6_d1}
                    }
                }

                div { class: "flex flex-col gap-8 items-start",
                    h2 { class: "text-2xl/22.5 font-bold text-text-primary", {tr.t7} }
                    p { class: "text-[15px]/22.5 font-normal text-text-secondary",
                        {tr.t7_d1}
                    }
                }

                div { class: "flex flex-col gap-8 items-start",
                    h2 { class: "text-2xl/22.5 font-bold text-text-primary", {tr.t8} }
                    p { class: "text-[15px]/22.5 font-normal text-text-secondary",
                        {tr.t8_d1}
                    }
                }

                p { class: "text-[15px]/22.5 font-normal text-text-secondary", {tr.t9} }

            } // end of this page

            Footer { lang }
        }
    }
}
