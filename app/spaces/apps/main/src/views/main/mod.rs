use crate::*;

mod i18n;
use i18n::AppMainTranslate;
use space_common::hooks::use_user_role;

fn app_description(app_type: SpaceAppType, tr: &AppMainTranslate) -> String {
    match app_type {
        SpaceAppType::IncentivePool => tr.app_description_incentive_pool.to_string(),
        SpaceAppType::File => tr.app_description_file.to_string(),
        _ => tr.app_description_default.to_string(),
    }
}

#[component]
pub fn AppMainPage(space_id: SpacePartition) -> Element {
    let tr: AppMainTranslate = use_translate();
    let space_apps_loader = use_space_apps(&space_id)?;
    let space_apps = space_apps_loader.read().clone();
    let installed_types: Vec<SpaceAppType> = space_apps.iter().map(|app| app.app_type).collect();

    let app_types: Vec<SpaceAppType> = SpaceAppType::VARIANTS
        .into_iter()
        .copied()
        .filter(|app_type| *app_type != SpaceAppType::General)
        .collect();
    let lang = use_language();

    let mut in_progress = use_signal(|| Option::<SpaceAppType>::None);

    let handle_toggle_app = move |app_type: SpaceAppType, is_installed: bool| {
        let space_id = space_id.clone();

        move |_| {
            if in_progress().is_some() {
                return;
            }

            in_progress.set(Some(app_type));

            let space_id = space_id.clone();

            spawn(async move {
                let action = if is_installed { "uninstall" } else { "install" };

                let result = if is_installed {
                    uninstall_space_app(space_id.clone(), app_type)
                        .await
                        .map(|_| ())
                } else {
                    install_space_app(space_id.clone(), app_type)
                        .await
                        .map(|_| ())
                };

                in_progress.set(None);

                match result {
                    Ok(_) => invalidate_query(&["space_apps", &space_id.to_string()]),
                    Err(err) => error!("Failed to {} app ({:?}): {:?}", action, app_type, err),
                }
            });
        }
    };

    rsx! {
        div { class: "grid grid-cols-3 gap-5 content-start items-start w-full max-tablet:grid-cols-2 max-mobile:grid-cols-1",
            for app_type in app_types {
                {
                    let is_installed = installed_types.contains(&app_type);
                    let is_progress = match in_progress() {
                        Some(current) => current == app_type,
                        None => false,
                    };

                    rsx! {
                        div { class: "flex flex-col items-start w-full gap-[10px] rounded-t-[16px] bg-card p-[15px]",
                            div { class: "flex justify-center items-center w-10 h-10 bg-violet-500 rounded-[10px]",
                                icons::ratel::Chest {
                                    width: "24",
                                    height: "24",
                                    class: "text-font-primary [&>path]:fill-none [&>path]:stroke-current",
                                }
                            }
                            div { class: "flex flex-col items-start self-stretch gap-[6px]",
                                p { class: "font-bold leading-5 sp-dash-font-raleway text-[17px] tracking-[-0.18px] text-font-primary",
                                    {app_type.translate(&lang()).to_string()}
                                }
                                p { class: "font-medium leading-4 sp-dash-font-raleway text-[12px] tracking-[0] text-card-meta",
                                    {app_description(app_type, &tr)}
                                }
                            }
                            button {
                                class: if is_installed { "flex flex-col justify-center items-center self-stretch px-5 py-3 w-full font-bold leading-5 border gap-[10px] rounded-[10px] border-btn-outline-outline bg-btn-outline-bg text-btn-outline-text sp-dash-font-raleway text-[17px] tracking-[-0.18px]" } else { "flex flex-col justify-center items-center self-stretch px-5 py-3 w-full font-bold leading-5 gap-[10px] rounded-[10px] bg-btn-primary-bg text-btn-primary-text sp-dash-font-raleway text-[17px] tracking-[-0.18px]" },
                                disabled: in_progress().is_some(),
                                onclick: handle_toggle_app(app_type, is_installed),
                                if is_progress {
                                    if is_installed {
                                        {tr.uninstalling}
                                    } else {
                                        {tr.installing}
                                    }
                                } else if is_installed {
                                    {tr.uninstall}
                                } else {
                                    {tr.install}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn HomePage(space_id: SpacePartition) -> Element {
    let tr: AppMainTranslate = use_translate();
    let role_loader = use_user_role(&space_id)?;
    let role = role_loader.read().clone();
    match SpaceApp::can_view(role) {
        Ok(_) => rsx! {
            AppMainPage { space_id }
        },
        _ => rsx! {
            div { class: "flex justify-center items-center w-full h-full text-font-primary",
                {tr.no_permission}
            }
        },
    }
}
