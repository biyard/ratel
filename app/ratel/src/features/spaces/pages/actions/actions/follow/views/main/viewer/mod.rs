use crate::common::hooks::use_infinite_query;
use crate::common::use_toast;
use crate::features::spaces::pages::actions::actions::follow::components::FollowUserList;
use crate::features::spaces::pages::actions::actions::follow::components::follow_user_list::i18n::FollowUserListTranslate;
use crate::features::spaces::pages::actions::actions::follow::controllers::{
    follow_user, list_follow_users, unfollow_user,
};
use crate::features::spaces::pages::actions::actions::follow::*;
use crate::features::spaces::pages::actions::components::FullActionLayover;
mod i18n;
use i18n::FollowViewerTranslate;

#[component]
pub fn FollowViewerPage(
    space_id: ReadSignal<SpacePartition>,
    follow_id: ReadSignal<SpaceActionFollowEntityType>,
) -> Element {
    let tr: FollowViewerTranslate = use_translate();
    let list_tr: FollowUserListTranslate = use_translate();
    let nav = navigator();
    let nav_back = nav.clone();
    let mut toast = use_toast();
    let mut users_query =
        use_infinite_query(move |bookmark| list_follow_users(space_id(), bookmark))?;
    let users = users_query.items();
    let more_element = users_query.more_element();
    let on_refresh_list = {
        let mut users_query_refresh = users_query.clone();
        move |_| {
            users_query_refresh.refresh();
        }
    };
    let on_follow = {
        let space_id = space_id;
        let follow_id = follow_id;
        let on_refresh_list = on_refresh_list.clone();
        move |target_pk: Partition| {
            let mut on_refresh_list = on_refresh_list.clone();
            let mut toast = toast;
            spawn(async move {
                match follow_user(space_id(), follow_id(), target_pk).await {
                    Ok(_) => {
                        toast.info(list_tr.subscribed_toast.to_string());
                        on_refresh_list(());
                    }
                    Err(err) => {
                        toast.error(err);
                    }
                }
            });
        }
    };
    let on_unfollow = {
        let space_id = space_id;
        let follow_id = follow_id;
        let on_refresh_list = on_refresh_list.clone();
        move |target_pk: Partition| {
            let mut on_refresh_list = on_refresh_list.clone();
            let mut toast = toast;
            spawn(async move {
                match unfollow_user(space_id(), follow_id(), target_pk).await {
                    Ok(_) => {
                        toast.info(list_tr.unsubscribed_toast.to_string());
                        on_refresh_list(());
                    }
                    Err(err) => {
                        toast.error(err);
                    }
                }
            });
        }
    };

    rsx! {
        FullActionLayover {
            bottom_right: rsx! {
                Button {
                    style: ButtonStyle::Outline,
                    shape: ButtonShape::Square,
                    class: "min-w-[120px]",
                    onclick: move |_| {
                        nav_back.push(format!("/spaces/{}/actions", space_id()));
                    },
                    {tr.btn_back}
                }
            },
            div { class: "w-full",
                FollowUserList {
                    space_id: space_id(),
                    users,
                    can_delete: false,
                    on_refresh: on_refresh_list,
                    on_follow,
                    on_unfollow,
                    more_element,
                }
            }
        }
    }
}
