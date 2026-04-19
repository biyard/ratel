use crate::features::spaces::pages::actions::actions::follow::views::main::creator::FollowCreatorTranslate;
use crate::features::spaces::pages::actions::actions::follow::views::main::creator::FollowerSetting;
use crate::features::spaces::pages::actions::actions::follow::*;
use crate::features::spaces::pages::actions::controllers::{
    UpdateSpaceActionRequest, update_space_action,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum SaveStatus {
    Idle,
    Saving,
    Saved,
    Unsaved,
}

#[component]
pub fn TargetsCard(
    space_id: ReadSignal<SpacePartition>,
    follow_id: ReadSignal<SpaceActionFollowEntityType>,
    initial_title: String,
) -> Element {
    let tr: FollowCreatorTranslate = use_translate();
    let mut toast = use_toast();

    let action_id_str = follow_id().to_string();
    let mut title = use_signal(|| initial_title.clone());
    let mut last_saved_title = use_signal(|| initial_title);
    let mut save_version = use_signal(|| 0u64);
    let mut title_status = use_signal(|| SaveStatus::Idle);

    let action_id_for_save = action_id_str.clone();
    let mut save_title = use_callback(move |_: ()| {
        let current = title();
        if current == last_saved_title() {
            return;
        }
        title_status.set(SaveStatus::Saving);
        let action_id = action_id_for_save.clone();
        spawn(async move {
            let req = UpdateSpaceActionRequest::Title {
                title: current.clone(),
            };
            match update_space_action(space_id(), action_id, req).await {
                Ok(_) => {
                    last_saved_title.set(current);
                    title_status.set(SaveStatus::Saved);
                }
                Err(err) => {
                    error!("Failed to save follow title: {:?}", err);
                    title_status.set(SaveStatus::Unsaved);
                    toast.error(err);
                }
            }
        });
    });

    // Autosave with 3-second debounce.
    use_effect(move || {
        let version = save_version();
        if version == 0 {
            return;
        }
        spawn(async move {
            crate::common::utils::time::sleep(std::time::Duration::from_secs(3)).await;
            if save_version() != version {
                return;
            }
            if title() == last_saved_title() {
                return;
            }
            save_title.call(());
        });
    });

    rsx! {
        section { class: "pager__page", "data-page": "0",
            article { class: "page-card", "data-testid": "page-card-targets",
                header { class: "page-card__head",
                    div { class: "page-card__title-wrap",
                        span { class: "page-card__num", "{tr.card_index_1}" }
                        div {
                            h1 { class: "page-card__title", "{tr.card_targets_title}" }
                            div { class: "page-card__subtitle", "{tr.card_targets_subtitle}" }
                        }
                    }
                }

                // ── Title section ─────
                section { class: "section", "data-testid": "section-content",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_content_label}" }
                    }
                    div { class: "field",
                        div {
                            style: "display:flex;align-items:center;justify-content:space-between;gap:8px",
                            label { class: "field__label", "{tr.title_label}" }
                            AutosaveStatusBadge { status: title_status() }
                        }
                        input {
                            class: "input",
                            "data-testid": "follow-title",
                            placeholder: "{tr.title_placeholder}",
                            value: "{title()}",
                            oninput: move |e| {
                                title.set(e.value());
                                title_status.set(SaveStatus::Unsaved);
                                save_version.set(save_version() + 1);
                            },
                            onblur: move |_| save_title.call(()),
                        }
                    }
                }

                // ── Targets list ─────
                section { class: "section", "data-testid": "section-targets",
                    div { class: "section__head",
                        span { class: "section__label", "{tr.section_targets_label}" }
                        span { class: "section__hint", "{tr.section_targets_hint}" }
                    }
                    FollowerSetting { space_id }
                }
            }
        }
    }
}

#[component]
fn AutosaveStatusBadge(status: SaveStatus) -> Element {
    let tr: FollowCreatorTranslate = use_translate();
    let (label, modifier) = match status {
        SaveStatus::Idle => return rsx! {},
        SaveStatus::Saving => (tr.autosave_saving.to_string(), "autosave--saving"),
        SaveStatus::Saved => (tr.autosave_saved.to_string(), "autosave--saved"),
        SaveStatus::Unsaved => (tr.autosave_unsaved.to_string(), "autosave--unsaved"),
    };
    rsx! {
        span { class: "autosave {modifier}",
            span { class: "autosave__dot" }
            "{label}"
        }
    }
}
