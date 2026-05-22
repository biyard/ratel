use crate::features::admin::*;

translate! {
    AdminLayoutTranslate;
    brand_name: { en: "Admin", ko: "관리자" },
}

#[component]
pub fn AppLayout() -> Element {
    let tr: AdminLayoutTranslate = use_translate();
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let user = user_ctx().user.clone();
    let nav = use_navigator();

    match &user {
        Some(u) if u.user_type == UserType::SystemAdmin => {
            rsx! {
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
                        div { class: "ff-admin-arena__brand",
                            div { class: "ff-admin-arena__brand-logo", "R" }
                            div { class: "ff-admin-arena__brand-text",
                                div { class: "ff-admin-arena__brand-name", "{tr.brand_name}" }
                            }
                        }
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
