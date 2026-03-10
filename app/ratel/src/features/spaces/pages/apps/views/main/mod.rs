use super::*;

mod app_card;
mod i18n;

use app_card::AppCard;

use i18n::AppMainTranslate;

#[component]
pub fn SpaceAppsPage(space_id: SpacePartition) -> Element {
    let role = use_space_role()();
    if !role.can_edit() {
        return Err(Error::UnauthorizedAccess)?;
    }

    let tr: AppMainTranslate = use_translate();
    let navigator = use_navigator();
    let mut space_apps_loader = use_space_apps(&space_id)?;
    let space_apps = space_apps_loader.read().clone();
    let installed_types: Vec<SpaceAppType> = space_apps.iter().map(|app| app.app_type).collect();

    let installed_apps: Vec<SpaceAppType> = SpaceAppType::VARIANTS
        .into_iter()
        .copied()
        .filter(|app_type| app_type.is_default() || installed_types.contains(app_type))
        .collect();

    let available_apps: Vec<SpaceAppType> = SpaceAppType::VARIANTS
        .into_iter()
        .copied()
        .filter(|app_type| !app_type.is_default() && !installed_types.contains(app_type))
        .collect();

    let mut in_progress = use_signal(|| Option::<SpaceAppType>::None);
    let app_grid_class = "grid grid-cols-3 gap-[10px] content-start items-start w-full max-tablet:grid-cols-2 max-mobile:grid-cols-1";
    let toggle_space_id = space_id.clone();

    let handle_toggle_app = move |app_type: SpaceAppType, is_installed: bool| {
        let space_id = toggle_space_id.clone();
        let mut space_apps_loader = space_apps_loader;

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
                    Ok(_) => space_apps_loader.restart(),
                    Err(err) => error!("Failed to {} app ({:?}): {:?}", action, app_type, err),
                }
            });
        }
    };

    let handle_open_settings = move |settings_path: String| {
        let navigator = navigator.clone();

        move |_| {
            navigator.push(settings_path.clone());
        }
    };

    rsx! {
        div { class: "flex flex-col items-start w-full gap-[20px]",
            div { class: "flex flex-col gap-4 w-full",
                div { class: "flex flex-col gap-3 w-full",
                    p { class: "text-2xl font-bold leading-6 sp-dash-font-raleway text-font-primary",
                        {tr.installed_apps}
                    }
                    div { class: app_grid_class,
                        for app_type in installed_apps {
                            {
                                let settings_path = app_type.settings_path(&space_id);
                                let is_progress = match in_progress() {
                                    Some(current) => current == app_type,
                                    None => false,
                                };
                                let header_action = (!app_type.is_default())
                                    .then(|| {
                                        rsx! {
                                            Button {
                                                class: "flex justify-center items-center rounded-full disabled:opacity-50 disabled:cursor-not-allowed size-6 !p-0 text-font-body hover:!bg-white/8 hover:!text-font-primary",
                                                size: ButtonSize::Icon,
                                                style: ButtonStyle::Text,
                                                disabled: in_progress().is_some(),
                                                onclick: handle_toggle_app(app_type, true),
                                                "aria-label": "Uninstall app",
                                                icons::ratel::XMarkIcon { width: "16", height: "16", class: "w-4 h-4" }
                                            }
                                        }
                                    });
                                rsx! {
                                    AppCard { app_type, header_action,
                                        Button {
                                            class: "w-full",
                                            style: ButtonStyle::Primary,
                                            shape: ButtonShape::Square,
                                            disabled: is_progress,
                                            onclick: handle_open_settings(settings_path),
                                            {tr.setting}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if !available_apps.is_empty() {
                    div { class: "w-full h-px bg-web-card-divider" }
                    div { class: "flex flex-col gap-3 w-full",
                        p { class: "text-2xl font-bold leading-6 sp-dash-font-raleway text-font-primary",
                            {tr.available_apps}
                        }
                        div { class: app_grid_class,
                            for app_type in available_apps {
                                {
                                    let is_progress = match in_progress() {
                                        Some(current) => current == app_type,
                                        None => false,
                                    };

                                    rsx! {
                                        AppCard { app_type,
                                            Button {
                                                class: "w-full",
                                                style: ButtonStyle::Primary,
                                                shape: ButtonShape::Square,
                                                disabled: in_progress().is_some(),
                                                onclick: handle_toggle_app(app_type, false),
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
}
