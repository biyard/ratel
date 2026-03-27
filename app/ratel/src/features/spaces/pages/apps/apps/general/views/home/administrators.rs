use crate::spaces::space_common::providers::use_space_context;

use super::*;

#[component]
pub fn Administrators() -> Element {
    let tr: GeneralTranslate = use_translate();
    let ctx = use_space_context();
    let space = use_space();
    let mut toast = use_toast();

    let mut admins = use_loader(move || async move {
        let space_id = space().id;
        if ctx.role().is_admin() {
            list_space_admins(space_id).await
        } else {
            Ok(vec![])
        }
    })?;

    let mut new_username = use_signal(|| String::new());
    let mut adding = use_signal(|| false);

    rsx! {
        Card {
            div { class: "flex justify-between items-center self-stretch py-4 px-5 border-b border-separator",
                p { class: "font-bold font-raleway text-[24px]/[28px] tracking-[-0.24px] text-web-font-primary",
                    {tr.administrator}
                }
            }

            div { class: "flex flex-col items-start self-stretch p-5 gap-[10px] bg-card max-mobile:p-4",
                {
                    let admin_list = admins();
                    if admin_list.is_empty() {
                        rsx! {
                            p { class: "font-medium leading-6 font-raleway text-[15px] tracking-[0.5px] text-card-meta",
                                {tr.administrator_not_found}
                            }
                        }
                    } else {
                        rsx! {
                            for admin in admin_list.iter() {
                                AdministratorRow {
                                    key: "{admin.user_id}",
                                    name: admin.display_name.clone(),
                                    username: admin.username.clone(),
                                    profile_url: admin.profile_url.clone(),
                                    is_owner: admin.is_owner,
                                    on_remove: {
                                        let user_id = admin.user_id.clone();
                                        let space_id = space().id;
                                        move |_| {
                                            let user_id = user_id.clone();
                                            let space_id = space_id.clone();
                                            spawn(async move {
                                                match remove_space_admin(space_id, UserPartition(user_id)).await {
                                                    Ok(_) => {
                                                        admins.restart();
                                                    }
                                                    Err(err) => {
                                                        toast.error(err);
                                                    }
                                                }
                                            });
                                        }
                                    },
                                }
                            }
                        }
                    }
                }

                // Add admin form
                if ctx.role().is_admin() {
                    div { class: "flex gap-2 items-center w-full pt-2",
                        Input {
                            r#type: InputType::Text,
                            placeholder: tr.enter_username,
                            value: new_username(),
                            oninput: move |e: FormEvent| {
                                new_username.set(e.value());
                            },
                        }
                        Button {
                            size: ButtonSize::Small,
                            loading: adding(),
                            disabled: new_username().trim().is_empty() || adding(),
                            onclick: {
                                let space_id = space().id;
                                move |_| {
                                    let username = new_username().trim().to_string();
                                    let space_id = space_id.clone();
                                    async move {
                                        if username.is_empty() {
                                            return;
                                        }
                                        adding.set(true);
                                        match add_space_admin(space_id, AddSpaceAdminRequest { username }).await
                                        {
                                            Ok(_) => {
                                                new_username.set(String::new());
                                                admins.restart();
                                            }
                                            Err(err) => {
                                                toast.error(err);
                                            }
                                        }
                                        adding.set(false);
                                    }
                                }
                            },
                            {tr.add_admin}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AdministratorRow(
    name: String,
    username: String,
    profile_url: String,
    is_owner: bool,
    on_remove: EventHandler<MouseEvent>,
) -> Element {
    let tr: GeneralTranslate = use_translate();
    let profile = if profile_url.trim().is_empty() {
        DEFAULT_PROFILE_IMAGE.to_string()
    } else {
        profile_url
    };

    rsx! {
        div { class: "flex justify-between items-center py-3 px-4 w-full border rounded-[12px] border-separator bg-card max-tablet:flex-col max-tablet:items-start max-tablet:gap-3",
            div { class: "flex items-center gap-[10px]",
                img {
                    src: "{profile}",
                    alt: "{name}",
                    class: "object-cover w-12 h-12 rounded-full shrink-0",
                }

                div { class: "flex flex-col gap-1 items-start",
                    div { class: "flex gap-2 items-center",
                        p { class: "font-bold leading-5 font-raleway text-[17px] tracking-[-0.18px] text-web-font-primary",
                            "{name}"
                        }
                        if is_owner {
                            Badge {
                                color: BadgeColor::Blue,
                                size: BadgeSize::Normal,
                                {tr.owner}
                            }
                        } else {
                            Badge {
                                color: BadgeColor::Green,
                                size: BadgeSize::Normal,
                                {tr.admin}
                            }
                        }
                    }
                    p { class: "font-semibold leading-4 font-raleway text-[13px] tracking-[-0.14px] text-web-font-neutral",
                        "@{username}"
                    }
                }
            }

            if !is_owner {
                Button {
                    size: ButtonSize::Small,
                    style: ButtonStyle::Outline,
                    onclick: move |e| on_remove.call(e),
                    {tr.remove_admin}
                }
            }
        }
    }
}
