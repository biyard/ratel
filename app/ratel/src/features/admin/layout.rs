use crate::features::admin::*;

#[component]
pub fn AppLayout() -> Element {
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let user = user_ctx().user.clone();
    let nav = use_navigator();

    match &user {
        Some(u) if u.user_type == UserType::Admin => {
            rsx! {
                document::Stylesheet { href: asset!("./views/main/style.css") }
                section { class: "admin-arena",
                    // Sticky topbar — back button + ARENA-style title.
                    // Required because admin pages otherwise have no
                    // chrome of their own and there's no way out
                    // except browser-back.
                    header { class: "admin-arena__topbar",
                        button {
                            class: "admin-arena__back",
                            r#type: "button",
                            "data-testid": "admin-back",
                            aria_label: "Back",
                            onclick: move |_| {
                                nav.go_back();
                            },
                            svg {
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                polyline { points: "15 18 9 12 15 6" }
                            }
                            span { "Back" }
                        }
                        span { class: "admin-arena__title", "Admin" }
                    }
                    Outlet::<Route> {}
                }
            }
        }
        _ => {
            nav.push("/");
            rsx! {}
        }
    }
}
