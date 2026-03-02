use crate::*;

use ratel_auth::hooks::use_user_context;
use space_app_main::use_space_apps;
use space_common::components::{SpaceNav, SpaceNavItem, SpaceTop, SpaceTopLabel};
use space_common::hooks::{use_space_query, use_user_role};

use crate::menu::{AppMenuItem, get_app_menu_items};

use crate::i18n::SpaceAppLayoutTranslate;

#[component]
pub fn SpaceAppsLayout(space_id: SpacePartition) -> Element {
    let space_loader = use_space_query(&space_id)?;
    let space = space_loader.read().clone();

    let space_apps_loader = use_space_apps(&space_id)?;
    let space_apps = space_apps_loader.read().clone();
    let role_loader = use_user_role(&space_id)?;
    let role = role_loader.read().clone();

    let user_ctx = use_user_context();
    let user = user_ctx.read().user.clone();

    let tr: SpaceAppLayoutTranslate = use_translate();
    let _lang = use_language();

    let default_menu: Vec<AppMenuItem> = vec![
        AppMenuItem {
            name: tr.all_app.to_string(),
            icon: rsx! {
                icons::layouts::Apps {
                    width: "20",
                    height: "20",
                    class: "text-icon-primary [&>path]:stroke-current"
                }
            },
            route: Route::Main {
                space_id: space_id.clone(),
                rest: vec![],
            },
        },
        AppMenuItem {
            name: tr.general.to_string(),
            icon: rsx! {
                icons::settings::Settings2 {
                    width: "20",
                    height: "20",
                    class: "text-icon-primary [&>path]:fill-current [&>circle]:stroke-current"
                }
            },
            route: Route::General {
                space_id: space_id.clone(),
                rest: vec![],
            },
        },
    ];

    let app_menus = get_app_menu_items(space_id.clone(), &space_apps, &tr);
    let menus: Vec<SpaceNavItem> = default_menu
        .into_iter()
        .chain(app_menus.into_iter())
        .map(|item| SpaceNavItem {
            icon: item.icon,
            label: item.name,
            link: item.route.into(),
        })
        .collect();

    let labels = vec![SpaceTopLabel {
        label: space.title.clone(),
        link: None,
    }];

    rsx! {
        div { class: "grid overflow-hidden grid-cols-7 w-full h-screen bg-space-bg text-font-primary",
            SpaceNav {
                logo: "https://metadata.ratel.foundation/logos/logo.png",
                menus,
                user,
                role,
                // FIXME:
                login_handler: move |_| {},
            }
            div { class: "flex flex-col col-span-6 col-start-2 min-h-0",
                SpaceTop {
                    labels,
                    space_status: None,
                    show_participate_button: false,
                    on_participant: None,
                }
                div { class: "flex overflow-auto p-5 w-full top-[65px] grow bg-space-body-bg rounded-tl-[10px] h-[calc(100%-65px)]",
                    Outlet::<Route> {}
                }
            }
        }
        Layover {}
    }
}
