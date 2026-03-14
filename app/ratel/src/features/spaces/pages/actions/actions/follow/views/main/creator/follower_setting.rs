use crate::common::hooks::use_infinite_query;
use crate::features::spaces::pages::actions::actions::follow::components::{
    FollowUserInvite, FollowUserList,
};
use crate::features::spaces::pages::actions::actions::follow::controllers::list_follow_users;
use crate::features::spaces::pages::actions::actions::follow::*;

#[component]
pub fn FollowerSetting(space_id: ReadSignal<SpacePartition>) -> Element {
    let mut users_query =
        use_infinite_query(move |bookmark| list_follow_users(space_id(), bookmark))?;
    let users = users_query.items();
    let more_element = users_query.more_element();
    let on_refresh_invite = {
        let mut users_query_refresh = users_query.clone();
        move |_| {
            users_query_refresh.restart();
        }
    };
    let on_refresh_list = {
        let mut users_query_refresh = users_query.clone();
        move |_| {
            users_query_refresh.restart();
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
                more_element,
            }
        }
    }
}
