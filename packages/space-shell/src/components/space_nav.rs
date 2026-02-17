mod space_user_login;
mod space_user_profile;

pub use space_user_login::*;
pub use space_user_profile::*;

use crate::controllers::user::get_user;
use crate::*;

#[derive(Clone, PartialEq)]
pub struct SpaceNavItem {
    pub icon: Element,
    pub label: SpacePage,
    pub link: NavigationTarget,
}

impl TryFrom<Option<(Element, SpacePage, NavigationTarget)>> for SpaceNavItem {
    fn try_from(value: Option<(Element, SpacePage, NavigationTarget)>) -> Result<Self> {
        let value = value.ok_or_else(|| crate::Error::UnauthorizedAccess)?;

        Ok(Self {
            icon: value.0,
            label: value.1,
            link: value.2,
        })
    }

    type Error = crate::Error;
}

#[component]
pub fn SpaceNav(logo: String, menus: Vec<SpaceNavItem>) -> Element {
    let user = use_loader(move || get_user())?;

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
            if let Some(user) = user.read().as_ref() {
                SpaceUserProfile {
                    image: user.profile_url.clone(),
                    display_name: user.display_name.clone(),
                    user_role: SpaceUserRole::Creator,
                }
            } else {
                SpaceUserLogin {}
            }
        }
    }
}

#[component]
fn NavItem(item: SpaceNavItem) -> Element {
    // TODO: Apply i18n service
    let label = item.label.translate(&Language::En);

    rsx! {
        Link {
            class: "flex flex-row gap-2 items-center py-2 px-1 w-full text-sm font-medium rounded-sm text-text hover:bg-space-nav-item-hover",
            to: item.link,
            {item.icon}
            "{label}"
        }
    }
}
