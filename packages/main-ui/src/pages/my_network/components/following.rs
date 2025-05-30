use bdk::prelude::*;

use crate::pages::{FollowingController, NewsRightPanel, Sidebar};

pub fn FollowingPage(cx: Scope) -> Element {
    let following_users = use_signal(cx, || FollowingController::get_following_users());
    let suggested_users = use_signal(cx, || FollowingController::get_suggested_accounts());

    cx.render(rsx! {
        div { class: "flex bg-black text-white h-screen",
            Sidebar {  }
            Feed { following_users: following_users.clone(), suggested_users: suggested_users.clone() }
            NewsRightPanel {  } 
        }
    })
}