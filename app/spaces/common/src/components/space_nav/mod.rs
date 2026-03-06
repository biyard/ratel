use crate::*;
mod space_user_login;
mod space_user_profile;

use common::models::User;
pub use space_user_login::*;
pub use space_user_profile::*;

#[component]
pub fn SpaceNav(
    logo: String,
    menus: Vec<SpaceNavItem>,
    user: Option<User>,
    login_handler: EventHandler<()>,
    role: SpaceUserRole,
) -> Element {
    rsx! {
        div { class: "flex z-40 flex-col gap-2.5 justify-between pt-2.5 h-full divide-y shrink-0 divide-divider w-full",
            div { class: "flex flex-col gap-2.5 w-full pb-4",
                img { src: "{logo}", class: "mx-4 mt-5 mb-2.5 w-25" }

                div { class: "flex flex-col gap-1.5 items-start px-4 pt-2.5 font-bold text-xs/[14px]",
                    for item in menus.iter() {
                        NavItem { item: item.clone() }
                    }
                }
            }
            if let Some(user) = user {
                SpaceUserProfile {
                    image: user.profile_url.clone(),
                    display_name: user.display_name.clone(),
                    user_role: role,
                }
            } else {
                SpaceUserLogin { onclick: login_handler }
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
    let selected = if is_active {
        "bg-space-nav-item-hover"
    } else {
        ""
    };
    // NOTE: Link component does not support class attribute merging.
    rsx! {
        Link {
            class: "flex flex-row gap-2 items-center py-2 px-1 w-full text-sm font-medium rounded-sm text-text hover:bg-space-nav-item-hover {selected}",
            to: item.link,
            {item.icon}
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
