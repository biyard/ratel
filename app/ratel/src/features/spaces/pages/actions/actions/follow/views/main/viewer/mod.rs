use crate::common::hooks::use_infinite_query;
use crate::features::spaces::pages::actions::actions::follow::components::FollowUserList;
use crate::features::spaces::pages::actions::actions::follow::controllers::list_follow_users;
use crate::features::spaces::pages::actions::actions::follow::*;
use crate::features::spaces::pages::actions::components::FullActionLayover;
mod i18n;
use i18n::FollowViewerTranslate;

#[component]
pub fn FollowViewerPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: FollowViewerTranslate = use_translate();
    let nav = navigator();
    let nav_back = nav.clone();
    let mut users_query =
        use_infinite_query(move |bookmark| list_follow_users(space_id(), bookmark))?;
    let users = users_query.items();
    let more_element = users_query.more_element();
    let on_refresh_list = {
        let mut users_query_refresh = users_query.clone();
        move |_| {
            users_query_refresh.restart();
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
                    more_element,
                }
            }
        }
    }
}
