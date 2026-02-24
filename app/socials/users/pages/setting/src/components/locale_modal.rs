use crate::*;

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
                for (opt , label , data_pw) in options.into_iter() {
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
                                span { class: "w-5 flex items-center justify-center",
                                    if is_selected {
                                        crate::icons::validations::CheckCircle {
                                            width: "18",
                                            height: "18",
                                            class: "h-4.5 w-4.5 [&>circle]:hidden [&>path]:stroke-primary",
                                        }
                                    }
                                }
                                span { class: "text-text-primary font-medium text-sm/[16px]", "{label}" }
                            }
                        }
                    }
                }
            }

            div { class: "flex items-center justify-end gap-4 mt-6 px-5 pb-2",
                button {
                    class: "px-6 py-[12px] bg-cancel-button-bg font-bold text-base text-cancel-button-text hover:text-cancel-button-text/80 transition-colors",
                    onclick: on_cancel,
                    "Cancel"
                }
                button {
                    class: "px-8 py-[12px] font-bold text-base text-submit-button-text rounded-[10px] bg-submit-button-bg hover:bg-submit-button-bg/80",
                    onclick: move |_| on_save.call(selected()),
                    "Save"
                }
            }
        }
    }
}
