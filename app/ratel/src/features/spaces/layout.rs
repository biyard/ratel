use super::*;
use crate::features::auth::hooks::use_user_context;
use crate::features::auth::{LoginModal, UserContextStoreExt};
use crate::features::spaces::space_common::controllers::get_user_role;
use crate::features::spaces::space_common::hooks::use_space_query;
use crate::features::spaces::space_common::providers::SpaceContextProvider;
use crate::features::spaces::space_common::types::space_key;
use crate::features::spaces::space_common::{
    components::{SpaceNav, SpaceNavItem, SpaceTop, SpaceTopLabel},
    hooks::use_space_role,
};
use crate::features::spaces::*;

#[derive(Clone, Copy)]
pub struct SpaceLayoutUiContext {
    pub sidebar_visible: Signal<bool>,
}

pub fn use_space_layout_ui() -> SpaceLayoutUiContext {
    use_context()
}

#[component]
pub fn SpaceLayout(space_id: ReadSignal<SpacePartition>) -> Element {
    let mut ctx = SpaceContextProvider::init(space_id)?;

    use_context_provider(|| PopupService::new());
    use_context_provider(|| LayoverService::new());
    let sidebar_visible = use_signal(|| true);
    use_context_provider(move || SpaceLayoutUiContext { sidebar_visible });
    let role = ctx.current_role();
    let space = ctx.space();
    let lang = use_language();
    let show_sidebar = sidebar_visible();

    let mut query = use_query_store();
    let user_ctx = use_user_context();
    let user = user_ctx.read().user.clone();
    let anonymous_user_profile = if space.anonymous_participation
        && matches!(role, SpaceUserRole::Participant | SpaceUserRole::Candidate)
    {
        Some((
            space.participant_profile_url.clone().unwrap_or_else(|| {
                "https://metadata.ratel.foundation/ratel/default-profile.png".to_string()
            }),
            space
                .participant_display_name
                .clone()
                .unwrap_or_else(|| "Anonymous User".to_string()),
        ))
    } else {
        None
    };
    // Credentials now live at a top-level `/credentials` route (it shows
    // the current user's own credentials regardless of the URL).
    let credential_path = user.as_ref().map(|_user| "/credentials".to_string());
    let mut popup = use_popup();
    let tr: SpaceLayoutTranslate = use_translate();

    let is_mobile = use_is_mobile();

    let show_participate =
        matches!(role, SpaceUserRole::Viewer) && !space.participated && space.can_participate;

    let mut menus = vec![
        crate::features::spaces::pages::dashboard::get_nav_item(&space, role.clone()),
        crate::features::spaces::pages::overview::get_nav_item(&space, role.clone()),
        crate::features::spaces::pages::actions::get_nav_item(&space, role.clone()),
        crate::features::spaces::pages::apps::get_nav_item(&space, role.clone()),
        // crate::features::spaces::pages::rewards::get_nav_item(&space, role.clone()),
        // crate::features::spaces::pages::report::get_nav_item(&space, role.clone()),
    ]
    .into_iter()
    .flatten()
    .map(|item| SpaceNavItem {
        icon: item.0,
        label: item.1.translate(&lang()).to_string(),
        link: item.2,
    })
    .collect::<Vec<SpaceNavItem>>();
    let labels = vec![SpaceTopLabel {
        label: space.title.clone(),
        link: None,
    }];
    let space_status = space.status.clone();

    // The legacy "quick participate" callback used to call
    // `participate_space` directly without surfacing the informed-
    // consent checkbox. The flow has since been moved entirely into
    // `ParticipationCard` → join layover, and `SpaceTop` no longer
    // renders its own Participate button (`show_participate_button:
    // false` below). We deliberately drop the bypass closure here so
    // there is exactly one entry point for participation that always
    // requires explicit consent.

    let layout_class = if show_sidebar {
        "grid overflow-hidden grid-cols-1 w-full h-screen tablet:grid-cols-[250px_1fr] bg-space-bg text-web-font-primary max-tablet:flex max-tablet:flex-col max-tablet:overflow-visible max-tablet:!h-auto max-tablet:min-h-screen"
    } else {
        "grid overflow-hidden grid-cols-1 w-full h-screen bg-space-bg text-web-font-primary max-tablet:flex max-tablet:flex-col max-tablet:overflow-visible max-tablet:!h-auto max-tablet:min-h-screen"
    };

    let content_class = if show_sidebar {
        "flex overflow-x-hidden flex-col min-w-0 min-h-0 max-tablet:flex-1 max-tablet:order-0 max-tablet:min-h-[auto] max-tablet:pt-16"
    } else {
        "flex overflow-x-hidden flex-col min-w-0 min-h-0 max-tablet:flex-1 max-tablet:order-0 max-tablet:min-h-[auto]"
    };

    rsx! {
        SeoMeta {
            title: space.title.clone(),
            description: space.description(),
            image: if space.logo.is_empty() { "https://metadata.ratel.foundation/logos/logo-symbol.png".to_string() } else { space.logo.clone() },
        }
        div { class: "{layout_class}", "data-testid": "space-layout-container",
            if show_sidebar {
                SpaceNav {
                    class: "max-tablet:order-1",
                    space_id: space_id(),
                    logo: if space.logo.is_empty() { "https://metadata.ratel.foundation/logos/logo.png".to_string() } else { space.logo.clone() },
                    menus,
                    user,
                    anonymous_user_profile,
                    role,
                    real_role: ctx.role(),
                    on_role_change: move |new_role| {
                        ctx.current_role.set(new_role);
                    },
                    show_participation_card: show_participate,
                    credential_path: credential_path.clone(),
                    login_handler: move |_| {
                        let mut space_loader = ctx.space;
                        let mut role_loader = ctx.role;
                        let mut current_role = ctx.current_role;
                        let cb = Callback::new(move |_| {
                            space_loader.restart();
                            role_loader.restart();
                            spawn(async move {
                                if let Ok(new_role) = get_user_role(space_id()).await {
                                    current_role.set(new_role);
                                }
                            });
                        });
                        popup.open(rsx! {
                            LoginModal { on_success: cb }
                        }).with_title(tr.title);
                    },
                }
            }
            div { class: "{content_class}",
                if show_sidebar {
                    div {
                        class: "max-tablet:w-full max-tablet:fixed max-tablet:top-0 max-tablet:z-50 max-tablet:bg-space-bg",
                        "data-testid": "space-top-wrapper",
                        SpaceTop {
                            labels,
                            space_status,
                            show_participate_button: false,
                        }
                    }
                }
                if show_participate && is_mobile() {
                    div { class: "px-3 pt-2 max-mobile:px-2",
                        ParticipationCard {
                            space_id: space_id(),
                            credential_path: credential_path.clone(),
                            on_login: move |_| {
                                let mut space_loader = ctx.space;
                                let mut role_loader = ctx.role;
                                let mut current_role = ctx.current_role;
                                let cb = Callback::new(move |_| {
                                    space_loader.restart();
                                    role_loader.restart();
                                    spawn(async move {
                                        if let Ok(new_role) = get_user_role(space_id()).await {
                                            current_role.set(new_role);
                                        }
                                    });
                                });
                                popup.open(rsx! {
                                    LoginModal { on_success: cb }
                                }).with_title(tr.title);
                            },
                        }
                    }
                }
                div { class: "flex overflow-auto flex-1 p-5 w-full bg-background rounded-tl-[10px] max-tablet:rounded-tl-none max-tablet:p-3 max-mobile:p-2 max-tablet:overflow-visible",
                    SuspenseBoundary { Outlet::<Route> {} }
                }
            }
        }
        PopupZone {}
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
