use super::*;

#[component]
pub fn CreatorPage(space_id: SpacePartition) -> Element {
    let tr: AppMainTranslate = use_translate();
    let space_apps_loader = use_space_apps(&space_id)?;
    let space_apps = space_apps_loader.read().clone();
    let installed_types: Vec<SpaceAppType> = space_apps.iter().map(|app| app.app_type).collect();

    let default_apps: Vec<SpaceAppType> = SpaceAppType::VARIANTS
        .into_iter()
        .copied()
        .filter(|app_type| app_type.is_default())
        .collect();

    let installable_apps: Vec<SpaceAppType> = SpaceAppType::VARIANTS
        .into_iter()
        .copied()
        .filter(|app_type| !app_type.is_default())
        .collect();

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
        div { class: "flex flex-col gap-[20px] items-start w-full",
            p { class: "font-bold leading-[28px] sp-dash-font-raleway text-2xl text-font-primary",
                {tr.all_apps}
            }
            div { class: "grid grid-cols-3 gap-[10px] content-start items-start w-full max-tablet:grid-cols-2 max-mobile:grid-cols-1",
                for app_type in default_apps {
                    AppCard { app_type,
                        div { class: "flex flex-col items-start w-full px-4 py-3 border-t border-web-card-divider",
                            Button {
                                class: "w-full",
                                style: ButtonStyle::Secondary,
                                shape: ButtonShape::Square,
                                disabled: true,
                                {tr.default_apps}
                            }
                        }
                    }
                }
                for app_type in installable_apps {
                    {
                        let is_installed = installed_types.contains(&app_type);
                        let is_progress = match in_progress() {
                            Some(current) => current == app_type,
                            None => false,
                        };

                        rsx! {
                            AppCard { app_type,
                                div { class: "flex flex-col items-start w-full px-[15px] py-[12px] bg-card border-t border-web-card-divider rounded-b-[16px]",
                                    if is_installed {
                                        Button {
                                            class: "w-full",
                                            style: ButtonStyle::Outline,
                                            shape: ButtonShape::Square,
                                            disabled: in_progress().is_some(),
                                            onclick: handle_toggle_app(app_type, is_installed),
                                            if is_progress {
                                                {tr.uninstalling}
                                            } else {
                                                {tr.uninstall}
                                            }
                                        }
                                    } else {
                                        Button {
                                            class: "w-full",
                                            style: ButtonStyle::Primary,
                                            shape: ButtonShape::Square,
                                            disabled: in_progress().is_some(),
                                            onclick: handle_toggle_app(app_type, is_installed),
                                            if is_progress {
                                                {tr.installing}
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
        }
    }
}
