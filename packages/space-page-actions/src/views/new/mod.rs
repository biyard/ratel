use crate::*;

#[component]
pub fn NewActionPage(space_id: SpacePartition) -> Element {
    let tr: NewTranslate = use_translate();
    let action_types = ActionType::VARIANTS.iter().map(|e| {
        rsx! {
            ActionTypeItem { action_type: e.clone() }
        }
    });

    rsx! {
        div {
            id: "new-action-page",
            class: "flex flex-col gap-5 items-start w-full",
            h3 { "{tr.title}" }
            {action_types}
        }
    }
}

#[component]
pub fn ActionTypeItem(action_type: ActionType) -> Element {
    let lng = use_language();

    rsx! {
        div { class: "flex flex-col gap-2 p-4 border border-gray-300 rounded-lg w-full hover:bg-gray-100 cursor-pointer",
            span { class: "font-medium", "{action_type.translate(&lng)}" }
        }
    }
}

translate! {
    NewTranslate;

    title: {
        en: "Action Type",
        ko: "액션 유형",

    },
}


