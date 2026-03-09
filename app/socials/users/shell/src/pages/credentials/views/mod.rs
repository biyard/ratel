use super::components::{CodeInputModal, VerificationMethodModal};
use super::config::get;
use super::controllers::get_credentials::{CredentialResponse, get_credentials_handler};
use super::controllers::sign_attributes::{SignAttributesRequest, sign_attributes_handler};
use super::*;
use common::icons::ratel::*;
use dioxus::prelude::*;
use ratel_auth::hooks::use_user_context;

#[component]
pub fn Home(username: String) -> Element {
    let tr: CredentialsTranslate = use_translate();

    let user_ctx = use_user_context();
    let mut credential = use_loader(get_credentials_handler)?;
    let mut popup = use_popup();
    let mut toast = use_toast();
    let lang = use_language();

    let attributes = use_memo(move || {
        let credential = credential();

        let mut attributes: Vec<VerifiedAttribute> = Vec::new();

        if let Some(age) = credential.age {
            attributes.push(VerifiedAttribute {
                label: tr.age.to_string(),
                value: age.to_string(),
            });
        }
        if let Some(gender) = credential.gender.clone() {
            let gender_label = match gender.to_lowercase().as_str() {
                "male" => tr.male.to_string(),
                "female" => tr.female.to_string(),
                _ => gender.clone(),
            };
            attributes.push(VerifiedAttribute {
                label: tr.gender.to_string(),
                value: gender_label,
            });
        }
        if let Some(university) = credential.university.clone() {
            attributes.push(VerifiedAttribute {
                label: tr.university.to_string(),
                value: university,
            });
        }

        attributes
    });

    let attributes = attributes();
    let did = user_ctx().did();
    let lang = lang();

    rsx! {
        div { class: "flex flex-col gap-4 py-6 w-full",
            div {
                class: "flex overflow-hidden relative flex-col items-center py-6 px-4 rounded-lg gap-[17.5px]",
                style: "background: radial-gradient(circle at center, rgba(77, 92, 255, 0.5) 0%, rgba(30, 30, 30, 1) 100%);",
                BadgeCheckIcon { width: "40", height: "40" }
                div { class: "flex flex-col gap-1 items-center",
                    h2 { class: "text-lg font-bold text-white", {tr.vc} }
                    p { class: "text-sm text-neutral-300", "{tr.id}: {did}" }
                }
            }

            div { class: "flex flex-col gap-5 py-4 px-4 rounded-lg bg-component-bg",
                div { class: "text-base font-semibold text-text-primary", {tr.my_did} }

                if attributes.is_empty() {
                    div { class: "flex items-center py-3 px-4 rounded-lg border text-text-primary border-card-border",
                        {tr.no_data}
                    }
                }

                for attr in attributes {
                    VerifiedItem { label: attr.label, value: attr.value }
                }

                div { class: "flex justify-center",
                    button {
                        class: "text-primary hover:text-primary/60",
                        onclick: move |_| {
                            let inner_popup = popup.clone();
                            popup.open(rsx! {
                                VerificationMethodModal {
                                    on_identity_verify: move |new_credential| {
                                        credential.restart();
                                    },
                                }
                            });
                        },
                        {tr.verify}
                    }
                }
            }
        }
    }
}

#[component]
fn VerifiedItem(label: String, value: String) -> Element {
    rsx! {
        div { class: "flex flex-row gap-4 items-center p-4 rounded-lg border border-border-primary",
            if label == "Age" || label == "나이" {
                ProfileCardIcon { class: "[&>path]:fill-transparent [&_g>path]:fill-transparent" }
            } else {
                BranchUserIcon { class: "[&>path]:fill-transparent [&_g>path]:fill-transparent" }
            }
            div { class: "flex flex-col",

                p { class: "text-sm text-text-primary", "{label}" }
                p { class: "text-base font-medium text-text-primary", "{value}" }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
struct VerifiedAttribute {
    label: String,
    value: String,
}

translate! {
    CredentialsTranslate;

    vc: {
        en: "Verifiable Credential",
        ko: "검증 가능한 자격",
    },
    id: {
        en: "ID",
        ko: "아이디",
    },
    my_did: {
        en: "My DID",
        ko: "내 DID",
    },
    no_data: {
        en: "No verified credentials found",
        ko: "검증된 자격이 없습니다",
    },
    verify: {
        en: "Verify",
        ko: "인증하기",
    },
    verification_error: {
        en: "Verification failed",
        ko: "인증에 실패했습니다",
    },
    submit: {
        en: "Submit",
        ko: "제출",
    },
    age: {
        en: "Age",
        ko: "나이",
    },
    gender: {
        en: "Gender",
        ko: "성별",
    },
    university: {
        en: "University",
        ko: "대학교",
    },
    male: {
        en: "Male",
        ko: "남성",
    },
    female: {
        en: "Female",
        ko: "여성",
    },
}
