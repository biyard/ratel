use crate::views::TeamDaoTranslate;
use crate::*;
use dioxus::prelude::*;

#[component]
pub fn DaoRegistrationCard(
    eligible_count: usize,
    min_required: usize,
    can_register: bool,
    on_register: EventHandler<MouseEvent>,
) -> Element {
    let tr: TeamDaoTranslate = use_translate();

    let need_more = if eligible_count >= min_required {
        0
    } else {
        min_required - eligible_count
    };

    rsx! {
        div { class: "bg-card-bg dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
            h3 { class: "text-xl font-semibold text-text-primary mb-4", {tr.register_dao} }

            div { class: "bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-md p-4 mb-6",
                div { class: "flex gap-3",
                    icons::ratel::InformationCircleIcon {
                        width: "24",
                        height: "24",
                        class: "w-6 h-6 text-blue-600 dark:text-blue-400 shrink-0",
                    }
                    div {
                        h4 { class: "font-semibold text-blue-900 dark:text-blue-100 mb-2",
                            {tr.admin_requirements}
                        }
                        p { class: "text-sm text-blue-800 dark:text-blue-200 whitespace-pre-line",
                            {tr.admin_requirements_description}
                        }
                    }
                }
            }

            div { class: "flex items-center justify-between mb-6",
                div {
                    p { class: "text-sm text-text-secondary mb-1",
                        {tr.eligible_admins_count.replace("{{count}}", &eligible_count.to_string())}
                    }
                    p { class: "text-xs text-text-secondary", {tr.min_admins_required} }
                }

                if eligible_count >= min_required {
                    div { class: "px-3 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded-full text-sm font-medium",
                        "✓ Ready"
                    }
                } else {
                    div { class: "px-3 py-1 bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200 rounded-full text-sm font-medium",
                        {format!("Need {} more", need_more)}
                    }
                }
            }

            if !can_register && eligible_count < min_required {
                div { class: "mb-4 p-3 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-md",
                    p { class: "text-sm text-yellow-800 dark:text-yellow-200",
                        {tr.insufficient_admins}
                    }
                }
            }

            button {
                class: "w-full px-6 py-3 bg-primary text-white rounded-md font-medium hover:bg-primary-dark transition-colors disabled:opacity-50 disabled:cursor-not-allowed disabled:hover:bg-primary",
                disabled: !can_register,
                onclick: on_register,
                {tr.register_dao}
            }
        }
    }
}
