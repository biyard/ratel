use crate::common::hooks::use_infinite_query;
use crate::features::spaces::pages::actions::actions::subscription::components::{
    SubscriptionUserInvite, SubscriptionUserList,
};
use crate::features::spaces::pages::actions::actions::subscription::controllers::list_subscription_users;
use crate::features::spaces::pages::actions::actions::subscription::*;
mod i18n;
use i18n::SubscriptionCreatorTranslate;

#[component]
pub fn SubscriptionCreatorPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SubscriptionCreatorTranslate = use_translate();

    let mut ctx = use_space_actions_context();

    // FIXME: This is just example not work properly.
    ctx.mutate_title_and_tabs(
        tr.title,
        vec![
            SpaceActionSettingTab::new(
                tr.tab_general,
                Route::FollowActionPage {
                    space_id: space_id(),
                },
            ),
            SpaceActionSettingTab::new(
                tr.tab_common,
                Route::FollowActionPage {
                    space_id: space_id(),
                },
            ),
        ],
    );

    let nav = navigator();
    let on_back = move |_| {
        nav.go_back();
    };

    let mut users_query =
        use_infinite_query(move |bookmark| list_subscription_users(space_id(), bookmark))?;
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
            // Back button
            Button {
                class: "w-fit !p-0 !text-sm !font-medium !text-neutral-400 hover:!bg-transparent hover:!text-white"
                    .to_string(),
                style: ButtonStyle::Text,
                onclick: on_back,
                "← {tr.btn_back}"
            }

            SubscriptionUserInvite { space_id: space_id(), on_refresh: on_refresh_invite }

            SubscriptionUserList {
                space_id: space_id(),
                users,
                can_delete: true,
                on_refresh: on_refresh_list,
                more_element,
            }
        }
    }
}
