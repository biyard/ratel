use crate::features::spaces::pages::apps::controllers::{get_space_apps, install_space_app};
use crate::features::spaces::pages::apps::types::SpaceAppType;
use crate::features::spaces::pages::index::*;

#[component]
pub fn AppsSection(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let nav = use_navigator();
    let lang = use_language();
    let mut toast = use_toast();

    let mut apps_loader =
        use_loader(move || async move { get_space_apps(space_id()).await })?;
    let space_apps = apps_loader();
    let installed_types: Vec<SpaceAppType> = space_apps.iter().map(|app| app.app_type).collect();

    let installed_apps: Vec<SpaceAppType> = SpaceAppType::VARIANTS
        .iter()
        .copied()
        .filter(|t: &SpaceAppType| t.is_default() || installed_types.contains(t))
        .collect();
    let available_apps: Vec<SpaceAppType> = SpaceAppType::VARIANTS
        .iter()
        .copied()
        .filter(|t: &SpaceAppType| !t.is_default() && !installed_types.contains(t))
        .collect();

    let mut in_progress = use_signal(|| Option::<SpaceAppType>::None);

    let installed_count = installed_apps.len();
    let available_count = available_apps.len();

    rsx! {
        document::Stylesheet { href: asset!("./style.css") }

        section {
            class: "settings-section apps-section",
            "data-testid": "apps-section",
            div { class: "settings-section__sublabel",
                span { class: "settings-section__sublabel-text", "{tr.installed_apps}" }
                span { class: "settings-section__sublabel-count", "{installed_count}" }
            }

            for app_type in installed_apps {
                AppRow {
                    key: "{app_type:?}",
                    app_type,
                    installed: true,
                    in_progress: in_progress() == Some(app_type),
                    onclick: move |_| {
                        nav.push(app_type.settings_path(space_id()));
                    },
                }
            }

            if available_count > 0 {
                div { class: "settings-section__sublabel",
                    span { class: "settings-section__sublabel-text", "{tr.available_apps}" }
                    span { class: "settings-section__sublabel-count", "{available_count}" }
                }
                for app_type in available_apps {
                    AppRow {
                        key: "{app_type:?}",
                        app_type,
                        installed: false,
                        in_progress: in_progress() == Some(app_type),
                        onclick: move |_| async move {
                            if in_progress().is_some() {
                                return;
                            }
                            in_progress.set(Some(app_type));
                            let result = install_space_app(space_id(), app_type).await;
                            in_progress.set(None);
                            match result {
                                Ok(_) => apps_loader.restart(),
                                Err(err) => {
                                    toast.error(err);
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn AppRow(
    app_type: SpaceAppType,
    installed: bool,
    in_progress: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let lang = use_language();
    let icon_class = match app_type {
        SpaceAppType::General => "app-row__icon app-row__icon--general",
        SpaceAppType::File => "app-row__icon app-row__icon--file",
        SpaceAppType::Analyzes => "app-row__icon app-row__icon--analyze",
        SpaceAppType::Panels => "app-row__icon app-row__icon--panel",
        #[cfg(feature = "beta")]
        SpaceAppType::IncentivePool => "app-row__icon app-row__icon--general",
    };
    let name = app_type.translate(&lang());
    let desc = app_type.description(&lang());
    let app_slug = format!("{:?}", app_type).to_lowercase();
    let install_testid = format!("install-app-{}", app_slug);
    let settings_testid = format!("settings-app-{}", app_slug);

    rsx! {
        div { class: "app-row",
            div { class: "{icon_class}", {app_type.icon()} }
            div { class: "app-row__info",
                div { class: "app-row__name-line",
                    span { class: "app-row__name", "{name}" }
                    if installed {
                        span { class: "app-row__check",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "3",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                polyline { points: "20 6 9 17 4 12" }
                            }
                        }
                    }
                }
                div { class: "app-row__desc", "{desc}" }
            }
            div { class: "app-row__action",
                if installed {
                    button {
                        class: "app-row-btn",
                        "data-testid": "{settings_testid}",
                        disabled: in_progress,
                        onclick: move |e| onclick.call(e),
                        "{tr.settings_btn}"
                    }
                } else {
                    button {
                        class: "app-row-btn app-row-btn--install",
                        "data-testid": "{install_testid}",
                        disabled: in_progress,
                        onclick: move |e| onclick.call(e),
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            line {
                                x1: "12",
                                x2: "12",
                                y1: "5",
                                y2: "19",
                            }
                            line {
                                x1: "5",
                                x2: "19",
                                y1: "12",
                                y2: "12",
                            }
                        }
                        "{tr.install}"
                    }
                }
            }
        }
    }
}
