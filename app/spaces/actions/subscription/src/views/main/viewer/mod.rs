use crate::components::SubscriptionUserList;
use crate::controllers::list_subscription_users;
use crate::*;
use common::hooks::use_infinite_query;
mod i18n;
use i18n::SubscriptionViewerTranslate;

#[component]
pub fn SubscriptionViewerPage(space_id: SpacePartition) -> Element {
    let tr: SubscriptionViewerTranslate = use_translate();
    let nav = navigator();
    let on_back = move |_| {
        nav.go_back();
    };

    let space_id_signal = use_signal(move || space_id);
    let mut users_query =
        use_infinite_query(move |bookmark| list_subscription_users(space_id_signal(), bookmark))?;
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

            SubscriptionUserList {
                space_id: space_id_signal(),
                users,
                can_delete: false,
                on_refresh: on_refresh_list,
                more_element,
            }
        }
    }
}
