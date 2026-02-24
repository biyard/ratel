use crate::*;

#[component]
pub fn UserLayout(username: String) -> Element {
    let user_ctx = ratel_auth::hooks::use_user_context();
    let logged_in = user_ctx().user.is_some();

    rsx! {
        div { class: "flex overflow-x-hidden gap-5 justify-between py-3 mx-auto min-h-screen text-white bg-bg max-w-desktop max-tablet:px-2.5",
            if logged_in {
                UserSidemenu { username }
            }
            div { class: "flex flex-col grow px-5", Outlet::<Route> {} }
        }
    }
}
