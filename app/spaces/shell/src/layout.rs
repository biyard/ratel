use crate::{controllers::participate_space::participate_space, *};
use space_common::hooks::use_space_query;
use space_common::ratel_auth::hooks::use_user_context;
use space_common::ratel_auth::{LoginModal, UserContextStoreExt};
use space_common::types::space_key;
use space_common::{
    components::{SpaceNav, SpaceNavItem, SpaceTop, SpaceTopLabel},
    hooks::use_user_role,
};
#[component]
pub fn SpaceProvider(space_id: SpacePartition) -> Element {
    // user_role_provider(space_id.clone())?;
    // space_provider(space_id.clone())?;
    use_context_provider(|| LayoverService::new());
    rsx! {
        Outlet::<Route> {
        }
    }
}
#[component]
pub fn SpaceLayout(space_id: SpacePartition) -> Element {
    let role_loader = use_user_role(&space_id)?;
    let role = role_loader.read().clone();
    let space_loader = use_space_query(&space_id)?;
    let space = space_loader.read().clone();
    let lang = use_language();

    let user_ctx = use_user_context();
    let user = user_ctx.read().user.clone();
    let mut popup = use_popup();
    let tr: SpaceLayoutTranslate = use_translate();
    // FIXME

    let mut participate = use_action(participate_space);

    let show_participate = matches!(space.status, Some(common::SpaceStatus::InProgress))
        && !space.participated
        && space.can_participate;

    let menus = vec![
        dashboard::get_nav_item(space_id.clone(), role.clone()),
        overview::get_nav_item(space_id.clone(), role.clone()),
        actions::get_nav_item(space_id.clone(), role.clone()),
        apps::get_nav_item(space_id.clone(), role.clone()),
        report::get_nav_item(space_id.clone(), role.clone()),
    ]
    .into_iter()
    .map(|item| {
        if let Some(item) = item {
            Some(SpaceNavItem {
                icon: item.0,
                label: item.1.translate(&lang()).to_string(),
                link: item.2,
            })
        } else {
            None
        }
    })
    .flatten()
    .collect::<Vec<SpaceNavItem>>();
    let labels = vec![SpaceTopLabel {
        label: space.title.clone(),
        link: None,
    }];
    let space_status = space.status.clone();

    let on_participant = move |_| {
        let space_id = space_id.clone();
        let mut space = space.clone();
        async move {
            let space_detail = space_key(&space_id);
            participate.call(space_id).await;
            invalidate_query(&space_detail);
        }
    };

    rsx! {
        div { class: "grid overflow-hidden grid-cols-7 w-full h-screen bg-space-bg text-font-primary",
            SpaceNav {
                logo: "https://metadata.ratel.foundation/logos/logo.png",
                menus,
                user,
                role,
                login_handler: move |_| {
                    popup.open(rsx! {
                        LoginModal {}
                    }).with_title(tr.title);
                },
            }
            div { class: "flex flex-col col-span-6 col-start-2 min-h-0",
                SpaceTop {
                    labels,
                    space_status,
                    show_participate_button: show_participate,
                    on_participant,
                }
                div { class: "flex overflow-auto p-5 w-full top-[65px] grow bg-space-body-bg rounded-tl-[10px] h-[calc(100%-65px)]",
                    Outlet::<Route> {}
                }
            }
        }
        Layover {}
    }
}

translate! {
    SpaceLayoutTranslate;

    title: {
        en: "Join the Movement",
        ko: "로그인 및 회원가입",
    },
}
