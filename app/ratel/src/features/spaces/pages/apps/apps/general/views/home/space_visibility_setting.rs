use super::*;

#[component]
pub fn SpaceVisibilitySetting() -> Element {
    let mut space = use_space();
    let tr: GeneralTranslate = use_translate();
    let mut toast = use_toast();
    let mut loading = use_signal(|| false);

    let is_public = space().visibility == SpaceVisibility::Public;

    rsx! {
        Card {
            div { class: "flex justify-between items-center self-stretch py-4 px-5 border-b border-separator",
                p { class: "font-semibold text-center font-raleway text-[17px]/[20px] tracking-[-0.18px] text-web-font-primary",
                    {tr.space_visibility}
                }
            }

            div { class: "flex flex-col items-start self-stretch p-5 gap-[10px] bg-card max-mobile:p-4",
                p { class: "font-normal leading-6 font-raleway text-[15px] tracking-[0.5px] text-card-meta",
                    {tr.space_visibility_description}
                }

                div { class: "flex flex-row gap-3 w-full max-mobile:flex-col",
                    VisibilityOptionCard {
                        selected: is_public,
                        disabled: loading(),
                        label: tr.public_space,
                        description: tr.public_space_desc,
                        onclick: move |_| async move {
                            if loading() || is_public {
                                return;
                            }
                            loading.set(true);
                            let result = update_space(
                                    space().id,
                                    UpdateSpaceRequest::Visibility {
                                        visibility: SpaceVisibility::Public,
                                    },
                                )
                                .await;
                            loading.set(false);
                            match result {
                                Ok(_) => {
                                    space.with_mut(|s| s.visibility = SpaceVisibility::Public);
                                    toast.info(tr.visibility_updated_successfully);
                                }
                                Err(err) => {
                                    toast.error(err);
                                }
                            }
                        },
                        icons::internet_script::Internet { class: "w-5 h-5 [&>path]:stroke-primary [&>path]:fill-none [&>circle]:stroke-primary [&>circle]:fill-none [&>ellipse]:stroke-primary [&>ellipse]:fill-none" }
                    }

                    VisibilityOptionCard {
                        selected: !is_public,
                        disabled: loading(),
                        label: tr.private_space,
                        description: tr.private_space_desc,
                        onclick: move |_| async move {
                            if loading() || !is_public {
                                return;
                            }
                            loading.set(true);
                            let result = update_space(
                                    space().id,
                                    UpdateSpaceRequest::Visibility {
                                        visibility: SpaceVisibility::Private,
                                    },
                                )
                                .await;
                            loading.set(false);
                            match result {
                                Ok(_) => {
                                    space.with_mut(|s| s.visibility = SpaceVisibility::Private);
                                    toast.info(tr.visibility_updated_successfully);
                                }
                                Err(err) => {
                                    toast.error(err);
                                }
                            }
                        },
                        icons::security::Lock1 { class: "w-5 h-5 [&>path]:stroke-primary [&>path]:fill-none" }
                    }
                }
            }
        }
    }
}

#[component]
fn VisibilityOptionCard(
    selected: bool,
    disabled: bool,
    label: String,
    description: String,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "group flex-1 flex items-center gap-4 p-4 rounded-xl border cursor-pointer transition-colors border-border hover:border-text-tertiary aria-selected:border-primary aria-selected:bg-primary/10 disabled:opacity-50 disabled:pointer-events-none",
            "aria-selected": selected,
            disabled,
            onclick: move |e| onclick.call(e),
            div { class: "flex justify-center items-center w-10 h-10 rounded-full bg-primary/15 shrink-0",
                {children}
            }
            div { class: "flex flex-col gap-0.5 items-start",
                span { class: "text-sm font-semibold text-text-primary", "{label}" }
                span { class: "text-xs text-text-secondary", "{description}" }
            }
            div { class: "ml-auto shrink-0",
                div { class: "w-5 h-5 rounded-full border-2 flex items-center justify-center border-text-tertiary group-aria-selected:border-primary",
                    if selected {
                        div { class: "w-2.5 h-2.5 rounded-full bg-primary" }
                    }
                }
            }
        }
    }
}
