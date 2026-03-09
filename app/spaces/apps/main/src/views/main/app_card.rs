use crate::{views::main::i18n::AppMainTranslate, *};

pub fn app_description(app_type: SpaceAppType, tr: &AppMainTranslate) -> String {
    match app_type {
        SpaceAppType::IncentivePool => tr.app_description_incentive_pool.to_string(),
        SpaceAppType::File => tr.app_description_file.to_string(),
        SpaceAppType::General => tr.app_description_general.to_string(),
    }
}

#[component]
pub fn AppCard(
    app_type: SpaceAppType,
    #[props(optional)] header_action: Option<Element>,
    children: Option<Element>,
) -> Element {
    let lang = use_language();
    let tr: AppMainTranslate = use_translate();

    let description = app_description(app_type, &tr);
    let has_footer = children.is_some();
    let icon_bg = match app_type {
        SpaceAppType::General => "bg-green-500",
        SpaceAppType::IncentivePool => "bg-violet-500",
        SpaceAppType::File => "bg-amber-500",
    };

    let icon = match app_type {
        SpaceAppType::General => rsx! {
            icons::settings::Settings2 {
                width: "24",
                height: "24",
                class: "text-white [&>path]:fill-black [&>circle]:stroke-black",
            }
        },
        SpaceAppType::IncentivePool => rsx! {
            icons::ratel::Chest {
                width: "24",
                height: "24",
                class: "text-white [&>path]:fill-none [&>path]:stroke-black",
            }
        },
        SpaceAppType::File => rsx! {
            icons::file::File {
                width: "24",
                height: "24",
                class: "text-white [&>path]:stroke-black",
            }
        },
    };

    rsx! {
        SpaceCard { class: "flex flex-col items-start overflow-hidden rounded-lg !p-0".to_string(),
            div { class: "flex flex-col p-4 gap-2.5 w-full",
                div { class: "flex items-start justify-between w-full gap-3",
                    div { class: "flex justify-center items-center shrink-0 size-11 rounded-[10px] {icon_bg}",
                        {icon}
                    }
                    if let Some(header_action) = header_action {
                        div { class: "flex shrink-0 items-center justify-center", {header_action} }
                    }
                }
                p { class: "font-bold sp-dash-font-raleway text-sm text-font-primary",
                    {app_type.translate(&lang()).to_string()}
                }
                p { class: "font-medium h-8 leading-4 sp-dash-font-raleway text-xs text-font-body line-clamp-2",
                    {description}
                }
            }

            if has_footer {
                div { class: "flex flex-col items-start w-full px-4 py-3 border-t border-web-card-divider",
                    {children}
                }
            }
        }
    }
}
