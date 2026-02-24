use crate::components::{CodeInputModal, VerificationMethodModal};
use crate::controllers::get_credentials::{CredentialResponse, get_credentials_handler};
use crate::*;
use common::BadgeCheckIcon;
use common::ProfileCardIcon;
use dioxus::prelude::*;
use ratel_auth::hooks::use_user_context;

#[component]
pub fn Home(username: String) -> Element {
    let tr: CredentialsTranslate = use_translate();
    let user_ctx = use_user_context();
    let did = user_ctx()
        .user
        .as_ref()
        .map(|u| format!("did:web:ratel.foundation:{}", u.username))
        .unwrap_or_else(|| "-".to_string());

    let resource = use_server_future(move || async move { get_credentials_handler().await })?;

    let resolved = resource.suspend()?;
    let data = resolved.read();

    let credential = match data.as_ref() {
        Ok(data) => data.clone(),
        Err(_) => CredentialResponse::default(),
    };

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

    let mut method_modal_open = use_signal(|| false);
    let mut code_modal_open = use_signal(|| false);
    let mut code_value = use_signal(String::new);
    let mut code_error = use_signal(|| Option::<String>::None);

    rsx! {
        div { class: "flex flex-col gap-4 w-full py-6",
            div {
                class: "overflow-hidden relative py-6 gap-[17.5px] flex flex-col items-center rounded-lg px-4",
                style: "background: radial-gradient(circle at center, rgba(77, 92, 255, 0.5) 0%, rgba(30, 30, 30, 1) 100%);",
                BadgeCheckIcon {}
                div { class: "flex flex-col items-center gap-1",
                    h2 { class: "text-lg font-bold text-white", "{tr.vc}" }
                    p { class: "text-sm text-neutral-300", "{tr.id}: {did}" }
                }
            }

            div { class: "rounded-lg px-4 py-4 bg-component-bg flex flex-col gap-5",
                div { class: "text-base font-semibold text-text-primary", "{tr.my_did}" }

                if attributes.is_empty() {
                    div { class: "flex items-center text-text-primary border border-card-border rounded-lg px-4 py-3",
                        "{tr.no_data}"
                    }
                } else {
                    for attr in attributes {
                        VerifiedItem { label: attr.label, value: attr.value }
                    }
                }

                div { class: "flex justify-center",
                    button {
                        class: "text-primary hover:text-primary/60",
                        onclick: move |_| method_modal_open.set(true),
                        "{tr.verify}"
                    }
                }
            }
        }

        if *method_modal_open.read() {
            VerificationMethodModal {
                on_identity_verify: move |_| {
                    info!("identity verification clicked");
                    method_modal_open.set(false);
                },
                on_code_verify: move |_| {
                    info!("code verification clicked");
                    method_modal_open.set(false);
                    code_modal_open.set(true);
                },
                on_close: move |_| {
                    method_modal_open.set(false);
                },
                title: tr.select_verification_method.to_string(),
                identity_title: tr.identity_verification.to_string(),
                identity_desc: tr.identity_verification_desc.to_string(),
                code_title: tr.code_verification.to_string(),
                code_desc: tr.code_verification_desc.to_string(),
                cancel_label: tr.cancel.to_string(),
            }
        }

        if *code_modal_open.read() {
            CodeInputModal {
                code_value: code_value(),
                code_error: code_error(),
                on_code_change: move |e: FormEvent| code_value.set(e.value()),
                on_submit: move |_| {
                    let code = code_value().trim().to_string();
                    if code.is_empty() {
                        code_error.set(Some(tr.invalid_code.to_string()));
                        return;
                    }
                    info!("code submitted: {}", code);
                    code_error.set(None);
                    code_modal_open.set(false);
                },
                on_close: move |_| {
                    code_modal_open.set(false);
                },
                title: tr.enter_code.to_string(),
                placeholder: tr.code_placeholder.to_string(),
                cancel_label: tr.cancel.to_string(),
                submit_label: tr.submit.to_string(),
            }
        }
    }
}

#[component]
fn VerifiedItem(label: String, value: String) -> Element {
    rsx! {
        div { class: "flex flex-row items-center gap-4 p-4 rounded-lg border border-[var(--border-primary)]",
            if label == "Age" || label == "나이" {
                ProfileCardIcon {}
            } else {
                BranchUserIcon {}
            }
            div { class: "flex flex-col",

                p { class: "text-sm text-[var(--text-secondary)]", "{label}" }
                p { class: "text-base font-medium text-[var(--text-primary)]", "{value}" }
            }
        }
    }
}

#[derive(Clone)]
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
    select_verification_method: {
        en: "Select Verification Method",
        ko: "인증 방법 선택",
    },
    identity_verification: {
        en: "Identity Verification",
        ko: "본인 인증",
    },
    identity_verification_desc: {
        en: "Verify your identity with a real-name method.",
        ko: "실명 인증 방식으로 본인을 확인합니다.",
    },
    code_verification: {
        en: "Code Verification",
        ko: "코드 인증",
    },
    code_verification_desc: {
        en: "Enter the code you received to verify.",
        ko: "받은 코드를 입력해 인증합니다.",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    },
    enter_code: {
        en: "Enter Code",
        ko: "코드 입력",
    },
    code_placeholder: {
        en: "Enter verification code",
        ko: "인증 코드를 입력하세요",
    },
    invalid_code: {
        en: "Invalid code",
        ko: "유효하지 않은 코드입니다",
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
