use crate::*;

#[component]
pub fn VerificationMethodModal(
    on_identity_verify: EventHandler<MouseEvent>,
    on_code_verify: EventHandler<MouseEvent>,
    on_close: EventHandler<MouseEvent>,
    title: String,
    identity_title: String,
    identity_desc: String,
    code_title: String,
    code_desc: String,
    cancel_label: String,
) -> Element {
    rsx! {
        div { class: "flex fixed inset-0 z-50 justify-center items-center bg-black bg-opacity-50",
            div { class: "p-6 w-full max-w-md bg-white rounded-lg shadow-lg dark:bg-gray-800",
                h2 { class: "mb-6 text-xl font-bold text-modal-label-text", "{title}" }

                div { class: "flex flex-col gap-4",
                    button {
                        onclick: on_identity_verify,
                        class: "p-4 text-left rounded-lg border border-gray-300 transition-all dark:border-gray-600 hover:bg-blue-50 hover:border-blue-500 dark:hover:border-blue-500 dark:hover:bg-gray-700",
                        div { class: "mb-1 text-lg font-semibold text-gray-700 dark:text-gray-500",
                            "{identity_title}"
                        }
                        p { class: "text-sm text-gray-600 dark:text-gray-400", "{identity_desc}" }
                    }

                    button {
                        onclick: on_code_verify,
                        class: "p-4 text-left rounded-lg border border-gray-300 transition-all dark:border-gray-600 hover:bg-blue-50 hover:border-blue-500 dark:hover:border-blue-500 dark:hover:bg-gray-700",
                        div { class: "mb-1 text-lg font-semibold text-gray-700 dark:text-gray-500",
                            "{code_title}"
                        }
                        p { class: "text-sm text-gray-600 dark:text-gray-400", "{code_desc}" }
                    }
                }

                div { class: "flex justify-end mt-6",
                    button {
                        class: "hover:text-white text-neutral-500",
                        onclick: on_close,
                        "{cancel_label}"
                    }
                }
            }
        }
    }
}
