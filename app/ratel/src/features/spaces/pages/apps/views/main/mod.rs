use super::*;

mod app_card;
mod i18n;

use app_card::AppCard;

use i18n::AppMainTranslate;

fn app_test_id(app_type: SpaceAppType, action: &str) -> &'static str {
    match (app_type, action) {
        (SpaceAppType::Panels, "install") => "install-panels-app",
        (SpaceAppType::Panels, "setting") => "setting-panels-app",
        _ => "",
    }
}

#[component]
pub fn SpaceAppGrid(children: Element) -> Element {
    rsx! {
        div { class: "grid grid-cols-3 content-start items-start w-full gap-[10px] max-tablet:grid-cols-2 max-mobile:grid-cols-1",
            {children}
        }
    }
}

#[component]
pub fn SpaceAppsPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let mut ctx = use_space_apps_context();
    let role = ctx.role();

    let tr: AppMainTranslate = use_translate();
    let navigator = use_navigator();
    let space_apps = ctx.apps();
    let mut toast = use_toast();
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

    let handle_toggle_app = move |app_type: SpaceAppType, is_installed: bool| async move {
        if in_progress().is_some() {
            return;
        }

        in_progress.set(Some(app_type));

        let result = if is_installed {
            uninstall_space_app(space_id(), app_type).await.map(|_| ())
        } else {
            install_space_app(space_id(), app_type).await.map(|_| ())
        };

        in_progress.set(None);

        match result {
            Ok(_) => {
                ctx.apps.restart();
            }
            Err(err) => {
                toast.error(err);
            }
        }
    };

    if !role.can_edit() {
        return Err(Error::UnauthorizedAccess)?;
    }

    rsx! {
        div { class: "flex flex-col items-start w-full gap-[20px]",
            div { class: "flex flex-col gap-4 w-full",
                div { class: "flex flex-col gap-3 w-full",
                    p { class: "text-2xl font-bold leading-6 sp-dash-font-raleway text-font-primary",
                        {tr.installed_apps}
                    }
                    SpaceAppGrid {
                        for app_type in installed_apps {
                            {
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
                                                onclick: move |_| handle_toggle_app(app_type, true),
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
                                            "data-testid": app_test_id(app_type, "setting"),
                                            onclick: move |_| {
                                                let settings_path = app_type.settings_path(space_id());
                                                navigator.push(settings_path);
                                            },
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
                        SpaceAppGrid {
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
                                                "data-testid": app_test_id(app_type, "install"),
                                                onclick: move |_| handle_toggle_app(app_type, false),
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
