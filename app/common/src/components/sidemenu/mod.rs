use crate::*;

#[component]
pub fn SideMenuContainer(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2.5 w-62.5 max-mobile:hidden shrink-0", {children} }
    }
}

#[component]
pub fn SideMenuProfileCard(children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col gap-5 px-4 py-5 w-full border rounded-[10px] bg-card-bg border-card-border", {children} }
    }
}

#[component]
pub fn SideMenuNav(children: Element) -> Element {
    rsx! {
        nav { class: "py-5 px-3 w-full border rounded-[10px] bg-card-bg border-card-border text-text-primary", {children} }
    }
}

#[component]
pub fn SideMenuLink(to: String, label: &'static str, icon: Element) -> Element {
    rsx! {
        Link { class: "sidemenu-link text-text-primary", to,
            span { class: "w-6 h-6 inline-flex items-center justify-center", {icon} }
            span { "{label}" }
        }
    }
}
