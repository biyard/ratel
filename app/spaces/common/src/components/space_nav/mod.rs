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
) -> Element {
    rsx! {
        div { class: "flex left-0 top-14 z-40 flex-col col-span-1 gap-2.5 justify-between pt-2.5 h-screen divide-y transition-transform duration-300 ease-in-out -translate-x-full shrink-0 divide-divider tablet:top-0 tablet:translate-x-0",
            div { class: "flex flex-col gap-2.5 w-full",
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
                    user_role: SpaceUserRole::Creator,
                }
            } else {
                SpaceUserLogin { onclick: login_handler }
            }
        }
    }
}

#[component]
fn NavItem(item: SpaceNavItem) -> Element {
    rsx! {
        Link {
            class: "flex gap-2 items-center self-stretch px-1 py-2 w-full h-9 font-bold leading-normal sp-dash-font-raleway text-[14px] text-font-primary hover:bg-space-nav-item-hover",
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
