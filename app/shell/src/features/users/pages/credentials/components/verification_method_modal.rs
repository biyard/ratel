use ratel_auth::hooks::use_user_context;

use super::super::{controllers::CredentialResponse, *};
use super::CodeInputModal;

#[component]
pub fn VerificationMethodModal(on_identity_verify: EventHandler<CredentialResponse>) -> Element {
    let tr: VerificationMethodModalTranslate = use_translate();
    let mut popup = use_popup();
    let mut toast = use_toast();
    let user = use_user_context();
    let lang = use_language()();

    rsx! {
        div { class: "p-6 w-full max-w-md",
            h2 { class: "mb-6 text-xl font-bold text-modal-label-text", {tr.title} }

            div { class: "flex flex-col gap-4",
                button {
                    onclick: move |_| async move {
                        #[cfg(not(feature = "server"))]
                        {
                            let conf = super::super::config::get();
                            let prefix = user().user_id().unwrap();

                            match super::super::interop::verify_identity(
                                    conf.portone.store_id,
                                    conf.portone.inicis_channel_key,
                                    &prefix,
                                )
                                .await
                            {
                                Ok(updated) => {
                                    popup.close();
                                    on_identity_verify(updated);
                                }
                                Err(err) => {
                                    toast.error(err);
                                }
                            }

                        }

                    },
                    class: "p-4 text-left rounded-lg border border-gray-300 transition-all dark:border-gray-600 hover:bg-blue-50 hover:border-blue-500 dark:hover:border-blue-500 dark:hover:bg-gray-700",
                    div { class: "mb-1 text-lg font-semibold text-gray-700 dark:text-gray-500",
                        {tr.id_title}
                    }
                    p { class: "text-sm text-gray-600 dark:text-gray-400", {tr.id_description} }
                }

                button {
                    onclick: move |_| {
                        popup.open(rsx! {
                            CodeInputModal { on_submit: on_identity_verify }
                        });
                    },
                    class: "p-4 text-left rounded-lg border border-gray-300 transition-all dark:border-gray-600 hover:bg-blue-50 hover:border-blue-500 dark:hover:border-blue-500 dark:hover:bg-gray-700",
                    div { class: "mb-1 text-lg font-semibold text-gray-700 dark:text-gray-500",
                        {tr.code_title}
                    }
                    p { class: "text-sm text-gray-600 dark:text-gray-400", {tr.code_desc} }
                }
            }

            div { class: "flex justify-end mt-6",
                button {
                    class: "hover:text-white text-neutral-500",
                    onclick: move |_| {
                        popup.close();
                    },
                    {tr.cancel}
                }
            }
        }
    }
}

translate! {
    VerificationMethodModalTranslate;
    title: {
        en: "Select Verification Method",
        ko: "인증 방법 선택",
    },


    id_title: {
        en: "Identity Verification",
        ko: "본인 인증",
    },
    id_description: {
        en: "Verify your identity with a real-name method.",
        ko: "실명 인증 방식으로 본인을 확인합니다.",
    },
    code_title: {
        en: "Code Verification",
        ko: "코드 인증",
    },
    code_desc: {
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
