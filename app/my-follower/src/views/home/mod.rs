mod i18n;

use crate::components::{FollowList, FollowTab, FollowTabs, MyFollowerHeader};
use crate::controllers::{follow_user, list_followers, list_followings, unfollow_user};
use crate::*;
use common::hooks::use_infinite_query;
use common::use_toast;
use dioxus::prelude::*;
use i18n::MyFollowerTranslate;

#[component]
pub fn Home() -> Element {
    let tr: MyFollowerTranslate = use_translate();
    let mut selected = use_signal(|| FollowTab::Followings);
    let toast = use_toast();

    let mut followers_query = use_infinite_query(move |bookmark| list_followers(bookmark))?;
    let mut followings_query = use_infinite_query(move |bookmark| list_followings(bookmark))?;

    let followers_loading = followers_query.is_loading();
    let followers = followers_query.items();
    let followers_more = followers_query.more_element();

    let followings_loading = followings_query.is_loading();
    let followings = followings_query.items();
    let followings_more = followings_query.more_element();

    let on_follow = {
        let mut followers_query = followers_query.clone();
        let mut followings_query = followings_query.clone();
        let toast = toast.clone();
        move |target_pk: Partition| {
            let mut followers_query = followers_query.clone();
            let mut followings_query = followings_query.clone();
            let mut toast = toast.clone();
            spawn(async move {
                match follow_user(target_pk).await {
                    Ok(_) => {
                        toast.info(tr.follow_toast.to_string());
                        followers_query.restart();
                        followings_query.restart();
                    }
                    Err(err) => {
                        toast.error(err);
                    }
                }
            });
        }
    };

    let on_unfollow = {
        let mut followers_query = followers_query.clone();
        let mut followings_query = followings_query.clone();
        let toast = toast.clone();
        move |target_pk: Partition| {
            let mut followers_query = followers_query.clone();
            let mut followings_query = followings_query.clone();
            let mut toast = toast.clone();
            spawn(async move {
                match unfollow_user(target_pk).await {
                    Ok(_) => {
                        toast.info(tr.unfollow_toast.to_string());
                        followers_query.restart();
                        followings_query.restart();
                    }
                    Err(err) => {
                        toast.error(err);
                    }
                }
            });
        }
    };

    rsx! {
        div { class: "flex flex-col w-full max-w-desktop mx-auto gap-5 max-tablet:px-2.5",
            MyFollowerHeader {}

            FollowTabs { selected: selected(), on_select: move |tab| selected.set(tab) }

            if selected() == FollowTab::Followers {
                FollowList {
                    users: followers,
                    selected: FollowTab::Followers,
                    loading: followers_loading,
                    on_follow,
                    on_unfollow,
                    more_element: followers_more,
                }
            } else {
                FollowList {
                    users: followings,
                    selected: FollowTab::Followings,
                    loading: followings_loading,
                    on_follow,
                    on_unfollow,
                    more_element: followings_more,
                }
            }
        }
    }
}
