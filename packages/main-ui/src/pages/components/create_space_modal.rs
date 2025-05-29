use crate::components::selectbox::rounded_selectbox::RoundedSelectbox;
use bdk::prelude::*;
use dto::{TotalInfoSummary, by_components::icons::validations::Clear};

#[component]
pub fn CreateSpacePopup(
    lang: Language,
    users: Vec<TotalInfoSummary>,
    onsend: EventHandler<Vec<i64>>,
) -> Element {
    let tr: CreateSpacePopupTranslate = translate(&lang);
    let mut selected_user_ids = use_signal(|| vec![]);
    let mut selected_users: Signal<Vec<TotalInfoSummary>> = use_signal(|| vec![]);

    rsx! {
        div { class: "w-full max-w-400 mx-5 max-mobile:!max-w-full",
            div { class: "w-400 max-mobile:!w-full",
                div { class: "justify-start text-white font-bold text-[15px]/20", {tr.to} }
                div { class: "flex flex-wrap w-full gap-4 mb-20 mt-10",
                    for (i , user) in selected_users().iter().enumerate() {
                        SelectedUser {
                            nickname: user.clone().nickname,
                            email: user.clone().email,
                            onremove: move |_| {
                                selected_user_ids.with_mut(move |users| users.remove(i));
                                selected_users.with_mut(move |users| users.remove(i));
                            },
                        }
                    }
                }
                div { class: "justify-start text-white font-bold text-[15px]/20 mt-35",
                    {tr.suggested}
                }
                div { class: "mt-20 max-h-400 w-full overflow-y-auto gap-4",
                    for (_ , user) in users.iter().enumerate() {
                        Account {
                            profile: user.profile_url.clone(),
                            name: user.nickname.clone(),
                            email: user.email.clone(),
                            selected: selected_user_ids().contains(&user.id),
                            onchange: {
                                let user = user.clone();
                                move |_| {
                                    let mut ids = selected_user_ids();
                                    let mut users_vec = selected_users();
                                    if let Some(pos) = ids.iter().position(|id| *id == user.id) {
                                        ids.remove(pos);
                                        users_vec.remove(pos);
                                    } else {
                                        ids.push(user.id);
                                        users_vec.push(user.clone());
                                    }
                                    selected_user_ids.set(ids);
                                    selected_users.set(users_vec);
                                }
                            },
                        }
                    }
                }

                SendButton {
                    lang,
                    onclick: move |_| {
                        onsend.call(selected_user_ids());
                    },
                }
            }
        }
    }
}

#[component]
pub fn SendButton(lang: Language, onclick: EventHandler<MouseEvent>) -> Element {
    let tr: SendButtonTranslate = translate(&lang);
    rsx! {
        div {
            class: "cursor-pointer flex flex-col my-25 w-full justify-center items-center py-15 bg-primary rounded-[10px] font-bold text-base/19 text-[#000203]",
            onclick,
            {tr.send}
        }
    }
}

#[component]
pub fn Account(
    profile: String,
    name: String,
    email: String,
    selected: bool,
    onchange: EventHandler<MouseEvent>,
) -> Element {
    let mut image_failed = use_signal(|| false);
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center py-12",
            div { class: "flex flex-row flex-1 gap-11",
                if profile == "" || image_failed() {
                    div {
                        class: "w-48 h-48 min-w-48 min-h-48 rounded-full bg-neutral-400 aria_hidden:!hidden",
                        aria_hidden: !image_failed(),
                    }
                } else {
                    img {
                        class: "w-48 h-48 min-w-48 min-h-48 rounded-full object-cover aria_hidden:!hidden ",
                        aria_hidden: image_failed(),
                        src: profile,
                        onerror: move |_| {
                            image_failed.set(true);
                        },
                    }
                }
                div { class: "flex flex-col flex-1 gap-4 min-w-0",
                    div { class: "font-bold text-white text-[15px]/20 w-full  overflow-hidden ",
                        {name}
                    }
                    div { class: "font-semibold text-neutral-500 text-sm/20 w-full  overflow-hidden ",
                        {email}
                    }
                }
            }

            RoundedSelectbox { selected, onchange }
        }
    }
}

#[component]
pub fn SelectedUser(
    nickname: String,
    email: String,
    onremove: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-fit rounded-[100px] bg-primary pr-4 pl-12 gap-4",
            div { class: "font-medium text-neutral-900 text-[15px]/24",
                {format!("{}", if nickname == "" { email } else { nickname })}
            }
            div {
                class: "cursor-pointer w-24 h-24",
                onclick: move |e| {
                    onremove.call(e);
                },
                Clear {
                    class: "[&>path]:stroke-neutral-900",
                    width: "24",
                    height: "24",
                }
            }
        }
    }
}

translate! {
    SendButtonTranslate;

    send: {
        ko: "Send",
        en: "Send"
    }
}

translate! {
    CreateSpacePopupTranslate;

    to: {
        ko: "To",
        en: "To"
    }
    suggested: {
        ko: "Suggested",
        en: "Suggested"
    }
}
