use crate::features::spaces::pages::actions::actions::follow::components::FollowUserList;
use crate::features::spaces::pages::actions::actions::follow::controllers::list_follow_users;
use crate::features::spaces::pages::actions::actions::follow::*;
use crate::common::hooks::use_infinite_query;
mod i18n;
use i18n::FollowViewerTranslate;

#[component]
pub fn FollowViewerPage(space_id: SpacePartition) -> Element {
    let tr: FollowViewerTranslate = use_translate();
    let nav = navigator();
    let on_back = move |_| {
        nav.go_back();
    };

    let space_id_signal = use_signal(move || space_id);
    let mut users_query =
        use_infinite_query(move |bookmark| list_follow_users(space_id_signal(), bookmark))?;
    let users = users_query.items();
    let more_element = users_query.more_element();
    let on_refresh_list = {
        let mut users_query_refresh = users_query.clone();
        move |_| {
            users_query_refresh.restart();
        }
    };

    rsx! {
        div { class: "flex flex-col gap-4 w-full",
            // Back button
            Button {
                class: "w-fit !p-0 !text-sm !font-medium !text-neutral-400 hover:!bg-transparent hover:!text-white".to_string(),
                style: ButtonStyle::Text,
                onclick: on_back,
                "← {tr.btn_back}"
            }

            FollowUserList {
                space_id: space_id_signal(),
                users,
                can_delete: false,
                on_refresh: on_refresh_list,
                more_element,
            }
        }
    }
}
