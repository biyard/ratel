use crate::features::spaces::pages::actions::actions::follow::components::follow_user_list::i18n::FollowUserListTranslate;
use crate::features::spaces::pages::actions::actions::follow::components::{
    FollowFullCard, FollowSmallCard,
};
use crate::features::spaces::pages::actions::actions::follow::controllers::list_follow_users::FollowUserItem;
use crate::features::spaces::pages::actions::actions::follow::*;

translate! {
    FollowViewerCardsTranslate;

    highlighted_action: {
        en: "Highlighted Action",
        ko: "Highlighted Action"
    }
    sub_actions: {
        en: "Sub Actions",
        ko: "Sub Actions"
    }
    highlight_label: {
        en: "Highlight",
        ko: "Highlight"
    }
    brief_label: {
        en: "Brief",
        ko: "Brief"
    }
    highlight_copy: {
        en: "Start with the featured account for this follow action. This view is designed to give the main recommendation enough room to breathe.",
        ko: "이번 팔로우 액션에서 가장 먼저 보여줄 대표 계정입니다. 메인 추천 계정을 충분한 정보와 함께 크게 보여주는 형태입니다."
    }
    creator_copy: {
        en: "This account is currently pinned as the main profile in the action.",
        ko: "현재 이 액션에서 대표 프로필로 고정되어 있는 계정입니다."
    }
    sub_actions_copy: {
        en: "Browse the rest of the recommended accounts in a lighter grid layout.",
        ko: "나머지 추천 계정은 가벼운 그리드 카드 형태로 확인할 수 있습니다."
    }
    no_sub_actions: {
        en: "No additional profiles yet.",
        ko: "추가 프로필이 아직 없습니다."
    }
}

#[component]
pub fn FollowViewerCards(
    users: Vec<FollowUserItem>,
    on_follow: EventHandler<Partition>,
    on_unfollow: EventHandler<Partition>,
    more_element: Element,
) -> Element {
    let tr: FollowViewerCardsTranslate = use_translate();
    let list_tr: FollowUserListTranslate = use_translate();

    if users.is_empty() {
        return rsx! {
            SpaceCard { class: "w-full rounded-[20px]! px-6! py-10!",
                p { class: "text-sm font-medium text-foreground-muted", {list_tr.empty} }
            }
        };
    }

    let card_type = CardType::Full;
    let brief_users = users.clone();

    rsx! {
        match card_type {
            CardType::Full => rsx! {
                div { class: "flex w-full min-h-120 flex-col gap-3",
                    FollowFullCard {
                        users,
                        on_follow: on_follow.clone(),
                        on_unfollow: on_unfollow.clone(),
                        more_element,
                    }
                }
            },
            CardType::Small => rsx! {
                div { class: "flex w-full flex-col gap-3",
                    FollowSmallCard {
                        users: brief_users,
                        on_follow: on_follow.clone(),
                        on_unfollow: on_unfollow.clone(),
                        more_element,
                    }
                }
            },
        }
    }
}
