#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_translate::*;

use crate::components::button::secondary_botton::SecondaryButton;

use super::*;

#[component]
pub fn Support(lang: Language) -> Element {
    let tr: SupportTranslate = translate(&lang);

    rsx! {
        div {
            id: "support",
            class: "w-full max-w-1177 h-screen grid grid-cols-2 gap-50 max-[1177px]:mx-10",
            div { class: "col-span-1 w-full",
                SectionHeader {
                    section_name: tr.title,
                    title: tr.mission,
                    description: tr.description,
                    with_line: false,
                }
            }

            div { class: "col-span-1 w-full flex flex-col items-start gap-50",
                div { class: "col-span-1 w-full flex flex-col items-start gap-30",
                    div { class: "w-full grid grid-cols-2 gap-24 max-600:grid-cols-1",
                        LabeledInput {
                            label_name: tr.label_first_name,
                            placeholder: tr.placeholder_first_name,
                            oninput: |e| {
                                btracing::debug!("First name: {}", e);
                            },
                        }
                        LabeledInput {
                            label_name: tr.label_last_name,
                            placeholder: tr.placeholder_last_name,
                            oninput: |e| {
                                btracing::debug!("First name: {}", e);
                            },
                        }
                    }

                    LabeledInput {
                        class: "w-full",
                        label_name: tr.label_email,
                        placeholder: tr.placeholder_email,
                        oninput: |e| {
                            btracing::debug!("Email: {}", e);
                        },
                    }

                    LabeledInput {
                        class: "w-full",
                        label_name: tr.label_company,
                        placeholder: tr.placeholder_company,
                        oninput: |e| {
                            btracing::debug!("Company: {}", e);
                        },
                    }

                    // FIXME: dropdown box
                    LabeledInput {
                        class: "w-full",
                        label_name: tr.label_needs,
                        placeholder: tr.label_needs,
                        oninput: |e| {
                            btracing::debug!("Needs: {}", e);
                        },
                    }

                    LabeledInput {
                        class: "w-full",
                        label_name: tr.label_help,
                        placeholder: tr.placeholder_help,
                        oninput: |e| {
                            btracing::debug!("Help: {}", e);
                        },
                    }
                } // end of form

                SecondaryButton { onclick: |_| {}, {tr.btn_submit} }
            }
        }
    }
}

translate! {
    SupportTranslate;

    title: {
        ko: "Support",
        en: "Support",
    },

    mission: {
        ko: "지원이 필요하신가요?",
        en: "Need Support?",
    },

    description: {
        ko: "문의하기 위해 양식을 작성하세요.",
        en: "Fill in the form to get in touch.",
    }

    label_first_name: {
        ko: "이름",
        en: "First Name",
    }

    placeholder_first_name: {
        ko: "이름을 입력하세요",
        en: "First name",
    }

    label_last_name: {
        ko: "성",
        en: "Last Name",
    }

    placeholder_last_name: {
        ko: "성을 입력하세요",
        en: "Last name",
    }

    label_email: {
        ko: "이메일",
        en: "Email",
    }

    placeholder_email: {
        ko: "이메일을 입력하세요",
        en: "Your email",
    }

    label_company: {
        ko: "회사",
        en: "Company name",
    }

    placeholder_company: {
        ko: "회사명을 입력하세요",
        en: "Company",
    }

    label_needs: {
        ko: "어떤 주제가 필요하신가요?",
        en: "Which topic best fit your needs?",
    }

    label_help: {
        ko: "어떤 도움이 필요하신가요?",
        en: "How can we help? ",
    }

    placeholder_help: {
        ko: "도움이 필요한 내용을 공유해주세요.",
        en: "Please share what you want us to help.",
    }

    btn_submit: {
        ko: "제출하기",
        en: "SUBMIT",
    }
}
