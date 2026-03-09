use super::super::{
    controllers::{CredentialResponse, SignAttributesRequest, sign_attributes_handler},
    *,
};

#[component]
pub fn CodeInputModal(on_submit: EventHandler<CredentialResponse>) -> Element {
    let tr: CodeInputModalTranslate = use_translate();
    let mut popup = use_popup();
    let mut toast = use_toast();
    let lang = use_language();
    let mut code_value = use_signal(String::default);

    let lang = lang();

    rsx! {
        div { class: "p-6 w-full max-w-md",
            h2 { class: "mb-4 text-xl font-bold text-modal-label-text", {tr.title} }

            div { class: "mb-4",
                input {
                    r#type: "text",
                    value: {code_value()},
                    oninput: move |e| code_value.set(e.value()),
                    placeholder: {tr.placeholder},
                    class: "py-2 px-3 w-full rounded border border-gray-300 dark:bg-gray-700 dark:border-gray-600 text-neutral-500",
                }
            }

            div { class: "flex gap-2 justify-end",
                button {
                    class: "hover:text-white text-neutral-500",
                    onclick: move |_| {
                        popup.close();
                    },
                    {tr.cancel}
                }
                button {
                    class: "py-2 px-4 rounded-md bg-enable-button-bg text-enable-button-white-text",
                    onclick: move |evt| async move {
                        let code = code_value();
                        if code.is_empty() {
                            toast.error(common::Error::InvalidCodeInput);
                            return;
                        }

                        match sign_attributes_handler(SignAttributesRequest::Code {
                                code,
                            })
                            .await
                        {
                            Ok(updated) => {
                                popup.close();
                                on_submit(updated);
                            }
                            Err(e) => {
                                toast.error(e);
                            }
                        }

                    },
                    {tr.submit}
                }
            }
        }
    }
}

translate! {
    CodeInputModalTranslate;

    title: {
        en: "Enter Code",
        ko: "코드 입력",
    },
    placeholder: {
        en: "Enter verification code",
        ko: "인증 코드를 입력하세요",
    },
    cancel: {
        en: "Cancel",
        ko: "취소",
    },
    submit: {
        en: "Submit",
        ko: "제출",
    },

    invalid_code: {
        en: "Invalid code",
        ko: "유효하지 않은 코드입니다",
    },
}
