use crate::features::admin::*;

#[component]
pub fn AppLayout() -> Element {
    let user_ctx = crate::features::auth::hooks::use_user_context();
    let user = user_ctx().user.clone();
    let nav = use_navigator();

    match &user {
        Some(u) if u.user_type == UserType::Admin => {
            rsx! {
                Outlet::<Route> {}
            }
        }
        _ => {
            nav.push("/");
            rsx! {}
        }
    }
}
