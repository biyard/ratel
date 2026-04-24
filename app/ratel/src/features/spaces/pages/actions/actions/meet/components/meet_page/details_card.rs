use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::features::spaces::pages::actions::controllers::{
    UpdateSpaceActionRequest, update_space_action,
};
use crate::*;

#[component]
pub fn MeetDetailsCard() -> Element {
    let tr: MeetActionTranslate = use_translate();
    let mut toast = use_toast();
    let UseMeet {
        space_id,
        meet_id,
        meet,
        ..
    } = use_context::<UseMeet>();
    let current = meet().space_action.clone();
    let initial_title = current.title.clone();
    let initial_desc = current.description.clone();
    let mut title = use_signal(move || initial_title.clone());
    let mut desc = use_signal(move || initial_desc.clone());

    let save_title = move |_| {
        let value = title();
        spawn(async move {
            let action_id = meet_id().to_string();
            if let Err(e) = update_space_action(
                space_id(),
                action_id,
                UpdateSpaceActionRequest::Title { title: value },
            )
            .await
            {
                toast.error(e);
            }
        });
    };
    let save_desc = move |_| {
        let value = desc();
        spawn(async move {
            let action_id = meet_id().to_string();
            if let Err(e) = update_space_action(
                space_id(),
                action_id,
                UpdateSpaceActionRequest::Description { description: value },
            )
            .await
            {
                toast.error(e);
            }
        });
    };

    rsx! {
        section { class: "meet-card",
            header { class: "meet-card__head",
                h2 { class: "meet-card__title meet-card__title--meet", "{tr.details_label}" }
            }
            div { class: "field",
                label { class: "field__label", "{tr.details_title_label}" }
                input {
                    class: "field__input",
                    "data-testid": "meet-title-input",
                    placeholder: "{tr.details_title_placeholder}",
                    value: "{title}",
                    oninput: move |e| title.set(e.value()),
                    onfocusout: save_title,
                }
            }
            div { class: "field",
                label { class: "field__label", "{tr.details_description_label}" }
                textarea {
                    class: "field__textarea",
                    "data-testid": "meet-description-input",
                    placeholder: "{tr.details_description_placeholder}",
                    value: "{desc}",
                    oninput: move |e| desc.set(e.value()),
                    onfocusout: save_desc,
                }
            }
        }
    }
}
