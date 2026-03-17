use super::*;

#[component]
pub fn SpaceLogoSetting() -> Element {
    let mut space = use_space();
    let tr: GeneralTranslate = use_translate();
    let mut toast = use_toast();
    let mut loading = use_signal(|| false);

    let on_upload = move |url: String| {
        let url = url.clone();
        spawn(async move {
            if loading() {
                return;
            }
            loading.set(true);
            let space_id = space().id;
            let result = update_space(
                space_id,
                UpdateSpaceRequest::Logo {
                    logo: url.clone(),
                },
            )
            .await;
            loading.set(false);
            match result {
                Ok(_) => {
                    space.with_mut(|s| s.logo = url);
                    toast.info(tr.logo_updated_successfully);
                }
                Err(err) => {
                    toast.error(err);
                }
            }
        });
    };

    let logo = space().logo.clone();

    rsx! {
        Card {
            div { class: "flex justify-between items-center self-stretch py-4 px-5 border-b border-separator",
                p { class: "font-semibold text-center font-raleway text-[17px]/[20px] tracking-[-0.18px] text-web-font-primary",
                    {tr.space_logo}
                }
            }

            div { class: "flex flex-col items-start self-stretch p-5 gap-[10px] bg-card max-mobile:p-4",
                p { class: "font-normal leading-6 font-raleway text-[15px] tracking-[0.5px] text-card-meta",
                    {tr.space_logo_description}
                }

                div { class: "flex flex-row gap-4 items-center",
                    FileUploader { on_upload_success: on_upload,
                        if logo.is_empty() {
                            div { class: "flex justify-center items-center w-20 h-20 rounded-lg border-2 border-dashed cursor-pointer border-border hover:border-primary transition-colors",
                                span { class: "text-2xl text-card-meta", "+" }
                            }
                        } else {
                            img {
                                src: "{logo}",
                                class: "object-contain w-20 h-20 rounded-lg border cursor-pointer border-border hover:border-primary transition-colors",
                            }
                        }
                    }

                    if !logo.is_empty() {
                        Button {
                            class: "border border-web-error !bg-transparent !text-web-error hover:!bg-transparent hover:!border-web-error hover:!text-web-error",
                            style: ButtonStyle::Text,
                            loading: loading(),
                            onclick: move |_| async move {
                                if loading() {
                                    return;
                                }
                                loading.set(true);
                                let space_id = space().id;
                                let result = update_space(
                                        space_id,
                                        UpdateSpaceRequest::Logo {
                                            logo: String::new(),
                                        },
                                    )
                                    .await;
                                loading.set(false);
                                match result {
                                    Ok(_) => {
                                        space.with_mut(|s| s.logo = String::new());
                                        toast.info(tr.logo_updated_successfully);
                                    }
                                    Err(err) => {
                                        toast.error(err);
                                    }
                                }
                            },
                            icons::edit::Delete2 {
                                class: "w-4 h-4 [&>path]:stroke-current",
                            }
                        }
                    }
                }
            }
        }
    }
}
