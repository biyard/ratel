use crate::features::spaces::space_common::*;
mod mobile_more_panel;
mod participation_attributes_section;
mod participation_card;
mod participation_consent_section;
mod participation_credential_section;
mod participation_layover_header;
mod participation_requirements_layover;
mod participation_step_bar;
mod participation_verification_section;
mod prerequisite_actions_layover;
mod space_user_login;
mod space_user_profile;

use super::*;
use crate::common::models::User;
pub use mobile_more_panel::*;
pub use participation_attributes_section::*;
pub use participation_card::*;
pub use participation_consent_section::*;
pub use participation_credential_section::*;
pub use participation_layover_header::*;
pub use participation_requirements_layover::*;
pub use participation_step_bar::*;
pub use participation_verification_section::*;
pub use prerequisite_actions_layover::*;
pub use space_user_login::*;
pub use space_user_profile::*;

#[component]
pub fn SpaceNav(
    space_id: SpacePartition,
    logo: String,
    menus: Vec<SpaceNavItem>,
    user: Option<User>,
    anonymous_user_profile: Option<(String, String)>,
    login_handler: EventHandler<()>,
    role: SpaceUserRole,
    real_role: SpaceUserRole,
    on_role_change: EventHandler<SpaceUserRole>,
    show_participation_card: bool,
    credential_path: Option<String>,
    #[props(default)] class: String,
) -> Element {
    let mut show_more_panel = use_signal(|| false);
    let is_logged_in = user.is_some();

    rsx! {
        div {
            class: "flex z-40 flex-col gap-2.5 justify-between pt-2.5 w-full h-full shrink-0 divide-divider {class} max-tablet:flex-row max-tablet:h-16 max-tablet:items-stretch max-tablet:justify-around max-tablet:sticky max-tablet:bottom-0 max-tablet:bg-space-bg",
            "data-testid": "space-nav-root",
            div { class: "flex flex-col flex-1 gap-2.5 pb-4 w-full",
                Link {
                    to: Route::SpaceDashboardPage {
                        space_id: space_id.clone(),
                    },
                    class: "mx-4 mt-5 mb-2.5 max-tablet:hidden",
                    img { src: "{logo}", class: "w-25" }
                }

                div { class: "max-tablet:hidden",
                    if show_participation_card {
                        ParticipationCard {
                            space_id: space_id.clone(),
                            credential_path,
                            on_login: login_handler,
                        }
                    }
                }

                div { class: "flex flex-col gap-1.5 items-start px-4 pt-2.5 font-bold text-xs/[14px] max-tablet:flex-row max-tablet:items-stretch max-tablet:justify-around max-tablet:p-0",
                    for (idx , item) in menus.iter().enumerate() {
                        NavItem { key: "{idx}", item: item.clone() }
                    }
                    // Mobile-only "More" tab
                    MobileMoreTab {
                        is_open: show_more_panel(),
                        onclick: move |_| {
                            show_more_panel.set(!show_more_panel());
                        },
                    }
                }
            }

            if !show_participation_card {
                RankingWidgetWrapper { space_id }
            }
            Row {
                class: "max-tablet:hidden",
                main_axis_align: MainAxisAlign::Between,
                cross_axis_align: CrossAxisAlign::Center,

                if let Some(ref user) = user {
                    SpaceUserProfile {
                        image: anonymous_user_profile
                            .as_ref()
                            .map(|(image, _)| image.clone())
                            .unwrap_or_else(|| user.profile_url.clone()),
                        display_name: anonymous_user_profile
                            .as_ref()
                            .map(|(_, display_name)| display_name.clone())
                            .unwrap_or_else(|| user.display_name.clone()),
                        user_role: role,
                        real_role,
                        on_role_change,
                    }
                } else {
                    SpaceUserLogin { onclick: login_handler }
                }

                SpaceThemeToggle {}
            }
        }

        // Mobile "More" panel overlay
        if show_more_panel() {
            {
                let (mobile_image, mobile_name) = if let Some(ref user) = user {
                    let image = anonymous_user_profile
                        .as_ref()
                        .map(|(image, _)| image.clone())
                        .unwrap_or_else(|| user.profile_url.clone());
                    let name = anonymous_user_profile
                        .as_ref()
                        .map(|(_, name)| name.clone())
                        .unwrap_or_else(|| user.display_name.clone());
                    (image, name)
                } else {
                    (String::new(), String::new())
                };
                rsx! {
                    MobileMorePanel {
                        is_logged_in,
                        user_image: mobile_image,
                        user_display_name: mobile_name,
                        user_role: role,
                        on_close: move |_| {
                            show_more_panel.set(false);
                        },
                        on_login: move |_| {
                            show_more_panel.set(false);
                            login_handler.call(());
                        },
                    }
                }
            }
        }
    }
}

#[component]
fn NavItem(item: SpaceNavItem) -> Element {
    let current_path = use_context::<dioxus::router::RouterContext>().full_route_string();
    let is_active = match &item.link {
        NavigationTarget::Internal(route) => current_path.starts_with(&route.to_string()),
        _ => false,
    };
    // NOTE: Link component does not support class attribute merging.
    rsx! {
        Link {
            class: "flex flex-row flex-1 gap-2 items-center py-2 px-1 w-full text-sm font-medium rounded-sm text-text aria-selected:bg-space-nav-item-selected max-tablet:flex-col max-tablet:gap-0.5 aria-selected:text-primary max-tablet:aria-selected:bg-transparent max-tablet:py-0 hover:bg-space-nav-item-hover",
            "aria-selected": is_active,
            to: item.link,
            div { class: "max-tablet:h-6 max-tablet:w-6 max-tablet:flex max-tablet:items-center max-tablet:justify-center",
                {item.icon}
            }
            {item.label}
        }
    }
}
#[derive(Clone, PartialEq)]
pub struct SpaceNavItem {
    pub icon: Element,
    pub label: String,
    pub link: NavigationTarget,
}

#[component]
fn RankingWidgetWrapper(space_id: SpacePartition) -> Element {
    rsx! {
        div { class: "px-2 max-tablet:hidden",
            dioxus::prelude::SuspenseBoundary { fallback: |_| rsx! {},
                crate::features::activity::components::RankingWidget { space_id }
            }
        }
    }
}

#[component]
fn SpaceThemeToggle() -> Element {
    let mut theme_service = use_theme();
    let current = theme_service.current();

    let next = match current {
        Theme::Light => Theme::Dark,
        Theme::Dark => Theme::System,
        Theme::System => Theme::Light,
    };

    rsx! {
        button {
            class: "flex justify-center items-center p-1.5 rounded-lg transition-colors cursor-pointer hover:bg-space-nav-item-hover",
            onclick: move |_| {
                theme_service.set(next);
            },
            match current {
                Theme::Dark => rsx! {
                    Moon {
                        width: "18",
                        height: "18",
                        class: "[&>path]:stroke-current text-web-font-neutral",
                    }
                },
                Theme::Light => rsx! {
                    Sun {
                        width: "18",
                        height: "18",
                        class: "[&>path]:stroke-current [&>circle]:stroke-current text-web-font-neutral",
                    }
                },
                Theme::System => rsx! {
                    SunMoon {
                        width: "18",
                        height: "18",
                        class: "[&>path]:stroke-current text-web-font-neutral",
                    }
                },
            }
        }
    }
}
