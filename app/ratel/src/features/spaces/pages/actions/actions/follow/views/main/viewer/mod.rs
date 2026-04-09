use crate::common::hooks::use_infinite_query;
use crate::common::query::use_query_store;
use crate::common::use_toast;
use crate::features::spaces::pages::actions::actions::follow::components::follow_user_list::i18n::FollowUserListTranslate;
use crate::features::spaces::pages::actions::actions::follow::components::FollowUserList;
use crate::features::spaces::pages::actions::actions::follow::controllers::{
    follow_user, list_follow_users, unfollow_user,
};
use crate::features::spaces::pages::actions::actions::follow::*;
use crate::features::spaces::pages::actions::components::FullActionLayover;
use crate::features::spaces::pages::actions::gamification::components::completion_overlay::CompletionOverlay;
use crate::features::spaces::pages::actions::gamification::components::quest_briefing::QuestBriefing;
use crate::features::spaces::pages::actions::gamification::hooks::use_quest_briefing;
use crate::features::spaces::pages::actions::gamification::types::{
    QuestNodeStatus, QuestNodeView, XpGainResponse,
};
use crate::features::spaces::pages::actions::types::SpaceActionType;
use crate::features::spaces::space_common::types::{space_my_score_key, space_ranking_key};
mod i18n;
use i18n::FollowViewerTranslate;

#[component]
pub fn FollowViewerPage(
    space_id: ReadSignal<SpacePartition>,
    follow_id: ReadSignal<SpaceActionFollowEntityType>,
) -> Element {
    let tr: FollowViewerTranslate = use_translate();
    let list_tr: FollowUserListTranslate = use_translate();
    let (show_briefing, dismiss) = use_quest_briefing();
    let nav = navigator();
    let nav_back = nav.clone();
    let mut toast = use_toast();
    let mut completion_response: Signal<Option<XpGainResponse>> = use_signal(|| None);
    let mut query = use_query_store();
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
            spawn(async move {
                match follow_user(space_id(), follow_id(), target_pk).await {
                    Ok(xp_opt) => {
                        toast.info(list_tr.subscribed_toast.to_string());
                        on_refresh_list(());
                        query.invalidate(&space_ranking_key(&space_id()));
                        query.invalidate(&space_my_score_key(&space_id()));
                        if let Some(xp) = xp_opt {
                            completion_response.set(Some(xp));
                        }
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
            spawn(async move {
                match unfollow_user(space_id(), follow_id(), target_pk).await {
                    Ok(_) => {
                        toast.info(list_tr.unsubscribed_toast.to_string());
                        on_refresh_list(());
                        query.invalidate(&space_ranking_key(&space_id()));
                        query.invalidate(&space_my_score_key(&space_id()));
                    }
                    Err(err) => {
                        toast.error(err);
                    }
                }
            });
        }
    };

    if show_briefing {
        let node = QuestNodeView {
            id: follow_id().to_string(),
            action_type: SpaceActionType::Follow,
            title: String::new(),
            base_points: 0,
            projected_xp: 0,
            status: QuestNodeStatus::Active,
            depends_on: vec![],
            chapter_id: String::new(),
            started_at: None,
            ended_at: None,
            quiz_result: None,
        };
        rsx! {
            QuestBriefing {
                node,
                on_begin: move |_| dismiss.call(()),
                on_cancel: move |_| {
                    nav.go_back();
                },
            }
        }
    } else {
        rsx! {
            CompletionOverlay { response: completion_response }
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
}
