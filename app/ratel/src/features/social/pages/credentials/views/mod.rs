use super::config::get;
use super::controllers::get_credentials::{get_credentials_handler, CredentialResponse};
use super::controllers::sign_attributes::{sign_attributes_handler, SignAttributesRequest};
use super::*;
use crate::common::components::{
    Button, ButtonStyle, Card, CardDirection, CardVariant, Col, CrossAxisAlign, MainAxisAlign, Row,
};
use crate::common::icons::ratel::*;
use crate::features::auth::hooks::use_user_context;
use dioxus::prelude::*;

#[component]
pub fn Home(username: String) -> Element {
    let tr: CredentialsTranslate = use_translate();

    let user_ctx = use_user_context();
    let mut credential = use_loader(get_credentials_handler)?;
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
        div { class: "flex flex-col gap-4 px-4 py-6 mx-auto w-full max-w-desktop max-mobile:px-3 max-mobile:py-4",

            // Hero credential card — figma blue/dark radial gradient.
            // The gradient itself is identical to the figma spec; the
            // text colors are pinned to white because the dark backdrop
            // is the same in both light and dark themes.
            div {
                class: "flex overflow-hidden relative flex-col gap-[17.5px] items-center py-6 px-4 w-full rounded-lg",
                style: "background: radial-gradient(circle at center, rgba(77, 92, 255, 0.5) 0%, rgba(30, 30, 30, 1) 100%);",
                BadgeCheckIcon { width: "40", height: "40" }
                div { class: "flex flex-col gap-1 items-center w-full text-center",
                    h2 { class: "text-lg font-bold text-white", {tr.vc} }
                    p { class: "text-sm break-all text-neutral-300", "{tr.id}: {did}" }
                }
            }

            // Attribute list card.
            Card {
                variant: CardVariant::Outlined,
                direction: CardDirection::Col,
                class: "gap-5 py-4 px-4 w-full",
                div { class: "text-base font-semibold text-text-primary", {tr.my_did} }

                if attributes.is_empty() {
                    Card {
                        variant: CardVariant::Outlined,
                        direction: CardDirection::Row,
                        cross_axis_align: CrossAxisAlign::Center,
                        class: "py-3 px-4 w-full text-text-primary",
                        {tr.no_data}
                    }
                }

                for attr in attributes {
                    VerifiedItem { label: attr.label, value: attr.value }
                }

                Row {
                    class: "justify-center w-full",
                    main_axis_align: MainAxisAlign::Center,
                    Button {
                        style: ButtonStyle::Text,
                        class: "font-semibold text-primary! hover:text-primary/70!",
                        onclick: move |_| async move {
                            // PortOne verification is a browser-only flow
                            // (interop module is gated to non-server
                            // builds), so the entire body is cfg-gated.
                            #[cfg(not(feature = "server"))]
                            {
                                let conf = super::config::get();
                                let prefix = match user_ctx().user_id() {
                                    Some(id) => id,
                                    None => {
                                        toast.warn(tr.verification_error.to_string());
                                        return;
                                    }
                                };
                                match super::interop::verify_identity(
                                        conf.portone.store_id,
                                        conf.portone.inicis_channel_key,
                                        &prefix,
                                    )
                                    .await
                                {
                                    Ok(_updated) => {
                                        credential.restart();
                                    }
                                    Err(err) => {
                                        toast.error(err);
                                    }
                                }
                            }
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
    let is_age = label == "Age" || label == "나이";
    rsx! {
        Card {
            variant: CardVariant::Outlined,
            direction: CardDirection::Row,
            cross_axis_align: CrossAxisAlign::Center,
            class: "gap-4 p-4 w-full",
            if is_age {
                ProfileCardIcon { class: "shrink-0 [&>path]:fill-transparent [&_g>path]:fill-transparent [&>path]:stroke-icon-primary [&_g>path]:stroke-icon-primary" }
            } else {
                BranchUserIcon { class: "shrink-0 [&>path]:fill-transparent [&_g>path]:fill-transparent [&>path]:stroke-icon-primary [&_g>path]:stroke-icon-primary" }
            }
            Col { class: "gap-0.5 min-w-0",
                p { class: "text-sm truncate text-foreground-muted", "{label}" }
                p { class: "text-base font-medium truncate text-text-primary", "{value}" }
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
