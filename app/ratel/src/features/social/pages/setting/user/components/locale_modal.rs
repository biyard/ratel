use super::super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LocaleOption {
    En,
    Ko,
}

impl LocaleOption {
    pub fn action_label(self) -> &'static str {
        match self {
            LocaleOption::En => "English",
            LocaleOption::Ko => "Korean",
        }
    }
}

#[component]
pub fn LocaleModal(
    initial_locale: LocaleOption,
    on_cancel: EventHandler<MouseEvent>,
    on_save: EventHandler<LocaleOption>,
) -> Element {
    let mut selected = use_signal(|| initial_locale);
    let options = [
        (LocaleOption::Ko, "Korean (한국어)", "locale-option-ko"),
        (LocaleOption::En, "English (English)", "locale-option-en"),
    ];

    rsx! {
        div { class: "w-[420px]",
            div { class: "flex flex-col gap-2 px-5",
                for (opt, label, data_pw) in options.into_iter() {
                    {
                        let is_selected = selected() == opt;
                        let base =
                            "flex items-center gap-3 w-full text-left px-3 py-3 rounded-[10px] transition-colors border";
                        let class = if is_selected {
                            format!("{base} border-neutral-400 light:border-primary light:bg-primary/10")
                        } else {
                            format!("{base} border-modal-card-border bg-modal-card-bg")
                        };

                        rsx! {
                            button { key: "{data_pw}", class: "{class}", onclick: move |_| selected.set(opt),
                                span { class: "flex justify-center items-center w-5",
                                    if is_selected {
                                        super::super::icons::validations::CheckCircle {
                                            width: "18",
                                            height: "18",
                                            class: "h-4.5 w-4.5 [&>circle]:hidden [&>path]:stroke-primary",
                                        }
                                    }
                                }
                                span { class: "font-medium text-text-primary text-sm/[16px]", "{label}" }
                            }
                        }
                    }
                }
            }

            div { class: "flex gap-4 justify-end items-center px-5 pb-2 mt-6",
                button {
                    class: "px-6 text-base font-bold transition-colors py-[12px] bg-cancel-button-bg text-cancel-button-text hover:text-cancel-button-text/80",
                    onclick: on_cancel,
                    "Cancel"
                }
                button {
                    class: "px-8 text-base font-bold py-[12px] text-submit-button-text rounded-[10px] bg-submit-button-bg hover:bg-submit-button-bg/80",
                    onclick: move |_| on_save.call(selected()),
                    "Save"
                }
            }
        }
    }
}
