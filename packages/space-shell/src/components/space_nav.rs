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
    rsx! {
        div { class: "col-span-1 gap-2.5 shrink-0 flex flex-col divide-y divide-divider py-2.5 top-14 tablet:top-0 left-0 z-40 h-full transition-transform duration-300 ease-in-out -translate-x-full tablet:translate-x-0",
            img { src: "{logo}", class: "w-25 mt-5 mb-2.5 mx-4" }

            div { class: "flex flex-col gap-1.5 px-4 pt-2.5 items-start font-bold text-xs/[14px]",
                for item in menus.iter() {
                    NavItem { item: item.clone() }
                }
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
            class: "w-full flex flex-row items-center gap-2 px-1 py-2 hover:bg-space-nav-item-hover rounded-sm text-sm font-medium text-text",
            to: item.link,
            {item.icon}
            "{label}"
        }
    }
}
