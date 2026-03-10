use super::*;

#[component]
pub fn ViewerPage(space_id: SpacePartition) -> Element {
    let tr: AppMainTranslate = use_translate();
    let navigator = use_navigator();
    let space_apps_loader = use_space_apps(&space_id)?;
    let space_apps = space_apps_loader.read().clone();
    let installed_types: Vec<SpaceAppType> = space_apps.iter().map(|app| app.app_type).collect();
    let installed_apps: Vec<SpaceAppType> = SpaceAppType::VARIANTS
        .into_iter()
        .copied()
        .filter(|app_type| app_type.is_default() || installed_types.contains(app_type))
        .collect();

    let handle_open_settings = move |settings_path: String| {
        let navigator = navigator.clone();

        move |_| {
            navigator.push(settings_path.clone());
        }
    };

    rsx! {
        div { class: "flex flex-col gap-[20px] items-start w-full",
            p { class: "font-bold text-lg text-font-primary", {tr.all_apps} }
            div { class: "grid grid-cols-3 gap-[10px] content-start items-start w-full max-tablet:grid-cols-2 max-mobile:grid-cols-1",
                for app_type in installed_apps {
                    {
                        let settings_path = app_type.settings_path(&space_id);

                        rsx! {
                            AppCard { app_type,
                                Button {
                                    class: "w-full",
                                    style: ButtonStyle::Primary,
                                    shape: ButtonShape::Square,
                                    onclick: handle_open_settings(settings_path),
                                    {tr.setting}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
