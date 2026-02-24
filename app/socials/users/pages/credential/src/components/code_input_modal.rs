use crate::*;

#[component]
pub fn CodeInputModal(
    code_value: String,
    code_error: Option<String>,
    on_code_change: EventHandler<FormEvent>,
    on_submit: EventHandler<MouseEvent>,
    on_close: EventHandler<MouseEvent>,
    title: String,
    placeholder: String,
    cancel_label: String,
    submit_label: String,
) -> Element {
    rsx! {
        div { class: "flex fixed inset-0 z-50 justify-center items-center bg-black bg-opacity-50",
            div { class: "p-6 w-full max-w-md bg-white rounded-lg shadow-lg dark:bg-gray-800",
                h2 { class: "mb-4 text-xl font-bold text-modal-label-text", "{title}" }

                div { class: "mb-4",
                    input {
                        r#type: "text",
                        value: "{code_value}",
                        oninput: on_code_change,
                        placeholder: "{placeholder}",
                        class: "py-2 px-3 w-full rounded border border-gray-300 dark:bg-gray-700 dark:border-gray-600 text-neutral-500",
                    }
                }

                if let Some(err) = code_error {
                    div { class: "mb-4 text-sm text-red-500", "{err}" }
                }

                div { class: "flex gap-2 justify-end",
                    button {
                        class: "hover:text-white text-neutral-500",
                        onclick: on_close,
                        "{cancel_label}"
                    }
                    button {
                        class: "bg-enable-button-bg text-enable-button-white-text px-4 py-2 rounded-md",
                        onclick: on_submit,
                        "{submit_label}"
                    }
                }
            }
        }
    }
}
