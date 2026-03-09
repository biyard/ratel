use super::super::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThemeOption {
    Dark,
    Light,
}

impl ThemeOption {
    pub fn label(self) -> &'static str {
        match self {
            ThemeOption::Dark => "Dark Theme",
            ThemeOption::Light => "Light Theme",
        }
    }

    pub fn key(self) -> &'static str {
        match self {
            ThemeOption::Dark => "dark",
            ThemeOption::Light => "light",
        }
    }

    pub fn to_theme(self) -> Theme {
        match self {
            ThemeOption::Dark => Theme::Dark,
            ThemeOption::Light => Theme::Light,
        }
    }

    pub fn from_theme(theme: Theme) -> Self {
        match theme {
            Theme::Light => ThemeOption::Light,
            Theme::Dark => ThemeOption::Dark,
            Theme::System => ThemeOption::Dark,
        }
    }
}

#[component]
pub fn ThemeModal(
    initial_theme: ThemeOption,
    on_cancel: EventHandler<MouseEvent>,
    on_save: EventHandler<ThemeOption>,
    on_preview: EventHandler<ThemeOption>,
) -> Element {
    let mut selected = use_signal(|| initial_theme);
    let options = [ThemeOption::Dark, ThemeOption::Light];

    rsx! {
        div { class: "w-[420px]",
            div { class: "flex flex-col gap-2",
                for opt in options.into_iter() {
                    {
                        let is_selected = selected() == opt;
                        let key = opt.key();
                        let data_pw = format!("theme-option-{}", key);
                        let base = "flex items-center justify-between px-5 py-4 text-left rounded-[10px] light:bg-white";
                        let class = if is_selected {
                            format!(
                                "{base} border border-neutral-400 light:border-primary light:bg-primary/10",
                            )
                        } else {
                            format!("{base} border-modal-card-border bg-modal-card-bg")
                        };
                        rsx! {
                            button {
                                key: "{key}",
                                class: "{class}",
                                onclick: move |_| {
                                    on_preview.call(opt);
                                    selected.set(opt);
                                },
                                span { class: "text-text-primary", "{opt.label()}" }
                                if is_selected {
                                    super::super::icons::validations::CheckCircle { class: "h-5 w-5 [&>circle]:hidden [&>path]:stroke-primary" }
                                }
                            }
                        }
                    }
                }
            }

            div { class: "flex flex-row justify-end gap-4 mt-4",
                button {
                    class: "px-10 py-[14.5px] bg-cancel-button-bg font-bold text-base text-cancel-button-text hover:text-cancel-button-text/80 transition-colors",
                    onclick: on_cancel,
                    "Cancel"
                }
                button {
                    class: "w-full py-[14.5px] font-bold text-base text-submit-button-text rounded-[10px] bg-submit-button-bg hover:bg-submit-button-bg/80",
                    onclick: move |_| on_save.call(selected()),
                    "Save"
                }
            }
        }
    }
}
