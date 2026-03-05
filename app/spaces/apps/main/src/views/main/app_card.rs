use crate::{views::main::i18n::AppMainTranslate, *};
// rsx! {
//                         div { class: "flex flex-col items-start w-full gap-[10px] rounded-t-[16px] bg-card p-[15px]",
//                             div { class: "flex justify-center items-center w-10 h-10 bg-violet-500 rounded-[10px]",
//                                 icons::ratel::Chest {
//                                     width: "24",
//                                     height: "24",
//                                     class: "text-font-primary [&>path]:fill-none [&>path]:stroke-current",
//                                 }
//                             }
//                             div { class: "flex flex-col items-start self-stretch gap-[6px]",
//                                 p { class: "font-bold leading-5 sp-dash-font-raleway text-[17px] tracking-[-0.18px] text-font-primary",
//                                     {app_type.translate(&lang()).to_string()}
//                                 }
//                                 p { class: "font-medium leading-4 sp-dash-font-raleway text-[12px] tracking-[0] text-card-meta",
//                                     {app_description(app_type, &tr)}
//                                 }
//                             }
//                             button {
//                                 class: if is_installed { "flex flex-col justify-center items-center self-stretch px-5 py-3 w-full font-bold leading-5 border gap-[10px] rounded-[10px] border-btn-outline-outline bg-btn-outline-bg text-btn-outline-text sp-dash-font-raleway text-[17px] tracking-[-0.18px]" } else { "flex flex-col justify-center items-center self-stretch px-5 py-3 w-full font-bold leading-5 gap-[10px] rounded-[10px] bg-btn-primary-bg text-btn-primary-text sp-dash-font-raleway text-[17px] tracking-[-0.18px]" },
//                                 disabled: in_progress().is_some(),
//                                 onclick: handle_toggle_app(app_type, is_installed),
//                                 if is_progress {
//                                     if is_installed {
//                                         {tr.uninstalling}
//                                     } else {
//                                         {tr.installing}
//                                     }
//                                 } else if is_installed {
//                                     {tr.uninstall}
//                                 } else {
//                                     {tr.install}
//                                 }
//                             }
//                         }

pub fn app_description(app_type: SpaceAppType, tr: &AppMainTranslate) -> String {
    match app_type {
        SpaceAppType::IncentivePool => tr.app_description_incentive_pool.to_string(),
        SpaceAppType::File => tr.app_description_file.to_string(),
        SpaceAppType::Reward => tr.app_description_reward.to_string(),
        SpaceAppType::General => tr.app_description_general.to_string(),
    }
}

#[component]
pub fn AppCard(
    app_type: SpaceAppType,
    #[props(default = false)] has_footer: bool,
    children: Element,
) -> Element {
    let lang = use_language();
    let tr: AppMainTranslate = use_translate();

    let description = app_description(app_type, &tr);

    let icon_bg = match app_type {
        SpaceAppType::General => "bg-green-500",
        SpaceAppType::IncentivePool => "bg-violet-500",
        SpaceAppType::File => "bg-amber-500",
        SpaceAppType::Reward => "bg-cyan-500",
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
        SpaceAppType::Reward => rsx! {
            icons::ratel::Thunder {
                width: "24",
                height: "24",
                class: "text-white [&>path]:stroke-black",
            }
        },
    };

    rsx! {
        div { class: "flex flex-col items-center w-full",
            div { class: if has_footer { "flex flex-col gap-[10px] items-start w-full rounded-t-[16px] bg-card p-[15px]" } else { "flex flex-col gap-[10px] items-start w-full rounded-[16px] bg-card p-[15px]" },
                div { class: "flex justify-center items-center w-[44px] h-[44px] rounded-[10px] {icon_bg}",
                    {icon}
                }
                p { class: "font-bold leading-[20px] sp-dash-font-raleway text-sm  text-font-primary",
                    {app_type.translate(&lang()).to_string()}
                }
                p { class: "font-medium min-h-[32px] leading-[16px] sp-dash-font-raleway text-xs  text-font-body line-clamp-2",
                    {description}
                }
            }
            {children}
        }
    }
}
