use crate::common::hooks::use_infinite_query;
use crate::common::use_toast;
use crate::features::spaces::pages::actions::actions::follow::components::{
    FollowUserInvite, FollowUserList,
};
use crate::features::spaces::pages::actions::actions::follow::components::follow_user_list::i18n::FollowUserListTranslate;
use crate::features::spaces::pages::actions::actions::follow::controllers::list_follow_users;
use crate::features::spaces::pages::actions::actions::follow::*;
use crate::features::my_follower::controllers::{
    follow_user as follow_my_follower_user, unfollow_user as unfollow_my_follower_user,
};

#[component]
pub fn FollowerSetting(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: FollowUserListTranslate = use_translate();
    let mut toast = use_toast();
    let mut users_query =
        use_infinite_query(move |bookmark| list_follow_users(space_id(), bookmark))?;
    let users = users_query.items();
    let more_element = users_query.more_element();
    let on_refresh_invite = {
        let mut users_query_refresh = users_query.clone();
        move |_| {
            users_query_refresh.refresh();
        }
    };
    let on_refresh_list = {
        let mut users_query_refresh = users_query.clone();
        move |_| {
            users_query_refresh.refresh();
        }
    };
    let on_follow = {
        let on_refresh_list = on_refresh_list.clone();
        move |target_pk: Partition| {
            let mut on_refresh_list = on_refresh_list.clone();
            let mut toast = toast;
            spawn(async move {
                match follow_my_follower_user(target_pk).await {
                    Ok(_) => {
                        toast.info(tr.subscribed_toast.to_string());
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
        let on_refresh_list = on_refresh_list.clone();
        move |target_pk: Partition| {
            let mut on_refresh_list = on_refresh_list.clone();
            let mut toast = toast;
            spawn(async move {
                match unfollow_my_follower_user(target_pk).await {
                    Ok(_) => {
                        toast.info(tr.unsubscribed_toast.to_string());
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
        div { class: "flex flex-col gap-4 w-full",
            FollowUserInvite { space_id: space_id(), on_refresh: on_refresh_invite }

            FollowUserList {
                space_id: space_id(),
                users,
                can_delete: true,
                on_refresh: on_refresh_list,
                on_follow,
                on_unfollow,
                more_element,
            }
        }
    }
}
