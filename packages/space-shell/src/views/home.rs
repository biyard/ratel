use crate::{
    controllers::user::{get_user, login},
    *,
};
// FIXME: REMOVE THIS PAGE
#[component]
pub fn Home() -> Element {
    let mut user = use_loader(move || get_user())?;

    let mut login = use_action(move || login());
    rsx! {
        button {
            class: "w-20 h-10 p-2 bg-blue-200 text-black",
            onclick: move |_| async move {
                login.call().await;
                user.restart();
            },
            "로그인"
        }
        {
            if let Some(user) = user.read().as_ref() {
                rsx! {
                    div { "{user.display_name}" }
                }
            } else {
                rsx! {
                    div { "유저를 찾을 수 없습니다" }
                }
            }
        }
    }
}
