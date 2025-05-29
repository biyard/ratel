use bdk::prelude::*;
use user::User;

#[component]
pub fn UserCard(cx: Scope, user: User, is_following: bool) -> Element {
    let is_following = use_signal(cx, || is_following);

    let toggle_follow = move |_| {
        is_following.with_mut(|f| *f = !*f);
    };

    cx.render(rsx! {
        div {
            class: "flex items-center justify-between p-4 border-b border-gray-700 transition-all duration-500 hover:bg-gray-800",
            div {
                class: "flex items-center space-x-4",
                div { class: "w-10 h-10 bg-gray-800 rounded-full" }
                div {
                    h4 { class: "text-white font-semibold", "{user.name}" }
                    p { class: "text-sm text-gray-400", "{user.position}" }
                }
            },
            button {
                class: "bg-white text-black px-4 py-1 rounded-full text-sm hover:bg-gray-200",
                onclick: toggle_follow,
                if *is_following.read() { "Following" } else { "+ Follow" }
            }
        }
    })
}