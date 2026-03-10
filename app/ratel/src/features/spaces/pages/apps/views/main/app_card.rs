use crate::features::spaces::pages::apps::{views::main::i18n::AppMainTranslate, *};

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
    onclick: EventHandler<MouseEvent>,
    action_label: String,
    #[props(default)] disabled: bool,
) -> Element {
    let lang = use_language();
    let tr: AppMainTranslate = use_translate();

    let description = app_description(app_type, &tr);
    let has_footer = children.is_some();

    let icon = app_type.icon();

    rsx! {
        SpaceCard { class: "flex overflow-hidden flex-col items-start rounded-lg !p-0".to_string(),
            div { class: "flex flex-col gap-2.5 p-4 w-full",
                div { class: "flex gap-3 justify-between items-start w-full",
                    div { class: "flex justify-center items-center shrink-0 size-11 rounded-[10px] {app_type}",
                        {icon}
                    }
                    if let Some(header_action) = header_action {
                        div { class: "flex justify-center items-center shrink-0", {header_action} }
                    }
                }
                p { class: "text-sm font-bold sp-dash-font-raleway text-font-primary",
                    {app_type.translate(&lang()).to_string()}
                }
                p { class: "h-8 text-xs font-medium leading-4 sp-dash-font-raleway text-font-body line-clamp-2",
                    {description}
                }
            }

            if has_footer {
                div { class: "flex flex-col items-start py-3 px-4 w-full border-t border-web-card-divider",
                    Button {
                        class: "w-full",
                        style: ButtonStyle::Primary,
                        shape: ButtonShape::Square,
                        disabled,
                        onclick,
                        {action_label}
                    }
                }
            }
        }
    }
}
