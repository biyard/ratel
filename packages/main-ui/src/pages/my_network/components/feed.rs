use bdk::prelude::*;

use super::User;
// use crate::components::user_card::UserCard;
// use user::User;

#[derive(Props)]
pub struct FeedProps<'a> {
    following_users: Signal<Vec<User>>,
    suggested_users: Signal<Vec<User>>,
}

#[component]
pub fn Feed<'a>(cx: Scope<'a, FeedProps<'a>>) -> Element {
    cx.render(rsx! {
        div {
            class: "w-2/4 p-6 overflow-y-scroll",
            h1 { class: "text-2xl font-bold mb-4", "Following" }
            cx.props.following_users.read().iter().map(|user| rsx! {
                UserCard { user: user.clone(), is_following: true }
            })
            h2 { class: "text-xl font-bold mt-8 mb-4", "Suggested Accounts" }
            cx.props.suggested_users.read().iter().map(|user| rsx! {
                UserCard { user: user.clone(), is_following: false }
            })
        }
    })
}