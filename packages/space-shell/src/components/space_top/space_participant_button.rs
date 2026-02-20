use crate::*;
use crate::controllers::participate_space::participate_space;

#[component]
pub fn SpaceParticipantButton(
    space_id: SpacePartition,
    on_participated: EventHandler<()>,
) -> Element {
    let mut participate = use_action(participate_space);
    let tr: SpaceParticipantButtonTranslate = use_translate();

    let onclick = move |_| {
        let space_id = space_id.clone();
        let on_participated = on_participated.clone();
        async move {
            participate.call(space_id).await;
            on_participated.call(());
        }
    };

    rsx! {
        Button { style: ButtonStyle::Primary, onclick, {tr.label} }
    }
}

translate! {
    SpaceParticipantButtonTranslate;

    label: {
        en: "Participate",
        ko: "참여하기",
    },
}
