use bdk::prelude::{
    by_components::icons::{
        arrows::{ShapeArrowDown, ShapeArrowUp},
        security::Logout,
        validations::Add,
    },
    *,
};
use num_format::{Locale, ToFormattedString};

use crate::{
    components::icons::{Badge, Grade},
    pages::{components::SideRoundedBox, controller::AccountList},
};

#[component]
pub fn LeftSideProfile(
    lang: Language,

    email: String,
    name: String,
    profile: String,
    description: String,

    exp: i64,
    total_exp: i64,

    followers: i64,
    replies: i64,
    posts: i64,
    spaces: i64,
    votes: i64,
    surveys: i64,

    accounts: Vec<AccountList>,

    add_account: EventHandler<MouseEvent>,
    sign_out: EventHandler<MouseEvent>,
    edit_profile: EventHandler<MouseEvent>,
) -> Element {
    let mut is_clicked = use_signal(|| true);
    let mut is_profile_clicked = use_signal(|| false);

    rsx! {
        div {
            class: "aria-active:!hidden",
            "aria-active": !is_profile_clicked(),
            SideRoundedBox {
                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-col w-full gap-10",
                        div {
                            class: "cursor-pointer flex flex-col w-full justify-start items-start p-10 rounded-lg bg-neutral-800",
                            onclick: move |_| {
                                is_profile_clicked.set(!is_profile_clicked());
                            },
                            div { class: "font-semibold text-white text-sm/20", "{name}" }
                            div { class: "font-medium text-neutral-500 text-xs/14",
                                {email}
                            }
                        }

                        Account {
                            lang,
                            accounts,
                            add_account,
                            sign_out,
                        }
                    }
                }
            }
        }
        div { class: "aria-active:!hidden", "aria-active": is_profile_clicked(),
            SideRoundedBox {
                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-col w-full gap-20",
                        div {
                            class: "cursor-pointer flex flex-row justify-between items-center",
                            onclick: move |_| {
                                is_clicked.set(!is_clicked());
                            },

                            div { class: "flex flex-row w-fit justify-start items-center gap-4",
                                div { class: "font-bold text-white text-lg/21", {name} }

                                Badge {}
                            }

                            if is_clicked() {
                                div { class: "flex flex-row w-fit h-fit",
                                    ShapeArrowDown {
                                        class: "[&>path]:stroke-white [&>path]:fill-white",
                                        size: 14,
                                        fill: "white",
                                    }
                                }
                            } else {
                                div { class: "flex flex-row w-fit h-fit",
                                    ShapeArrowUp {
                                        class: "[&>path]:stroke-white [&>path]:fill-white",
                                        size: 14,
                                        fill: "white",
                                    }
                                }
                            }
                        }

                        if is_clicked() {
                            div { class: "flex flex-col w-full justify-start items-start gap-30",
                                Profile {
                                    lang,
                                    profile,
                                    description,
                                    clicked: is_profile_clicked(),
                                    onchange_clicked: move |clicked: bool| {
                                        is_profile_clicked.set(clicked);
                                    },
                                    edit_profile: move |e| {
                                        edit_profile.call(e);
                                    },
                                }
                                                        // Tier {
                            //     lang,
                            //     exp,
                            //     total_exp,
                            //     followers,
                            //     replies,
                            //     posts,
                            //     spaces,
                            //     votes,
                            //     surveys,
                            // }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Account(
    lang: Language,
    accounts: Vec<AccountList>,
    add_account: EventHandler<MouseEvent>,
    sign_out: EventHandler<MouseEvent>,
) -> Element {
    let tr: AccountTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start px-16 py-20 rounded-lg bg-neutral-800 gap-20",
            div { class: "flex flex-col w-full gap-20",
                div { class: "font-bold text-sm/16 text-neutral-500", {tr.switch_account} }

                div { class: "flex flex-col w-full justify-start items-start gap-12",
                    for account in accounts {
                        div {
                            class: "cursor-pointer flex flex-row w-full justify-start items-center gap-8",
                            onclick: move |_| {
                                tracing::debug!("change account button clicked");
                            },
                            img {
                                class: "w-20 h-20 rounded-full object-cover",
                                src: account.profile,
                            }
                            div { class: "font-normal text-white text-sm/20", {account.email} }
                        }
                    }
                }
            }
            div { class: "flex flex-row w-full h-1 bg-neutral-700" }
            div { class: "flex flex-col w-full justify-start items-start gap-12",
                div {
                    class: "cursor-pointer flex flex-row w-full justify-start items-center gap-4",
                    onclick: move |e| {
                        add_account.call(e);
                    },
                    Add {
                        class: "[&>path]:stroke-white",
                        width: "20",
                        height: "20",
                        fill: "white",
                    }
                    div { class: "font-bold text-white text-sm/16", {tr.add_another_account} }
                }
                div {
                    class: "cursor-pointer flex flex-row w-full justify-start items-center gap-4",
                    onclick: move |e| {
                        sign_out.call(e);
                    },
                    Logout {
                        class: "[&>path]:stroke-white",
                        width: "20",
                        height: "20",
                    }
                    div { class: "font-bold text-white text-sm/16", {tr.sign_out} }
                }
            }
        }
    }
}

#[component]
pub fn Tier(
    lang: Language,
    exp: i64,
    total_exp: i64,
    followers: i64,
    replies: i64,
    posts: i64,
    spaces: i64,
    votes: i64,
    surveys: i64,
) -> Element {
    let mut is_open = use_signal(|| false);

    let tr: TierTranslate = translate(&lang);

    let percent = if total_exp > 0 {
        (exp as f32 / total_exp as f32) * 100.0
    } else {
        0.0
    };

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-14",
            div {
                class: "cursor-pointer flex flex-col w-full justify-start items-start gap-10",
                onclick: move |_| {
                    is_open.set(!is_open());
                },
                div { class: "flex flex-row w-full justify-between items-center",
                    div { class: "font-bold text-white text-sm/16", {tr.tier} }
                    div { class: "flex flex-row w-fit justify-start items-center gap-4",
                        div { class: "font-semibold text-white text-sm/20", {tr.diamond} }
                        Grade {}
                    }
                }
                GaugeBar { gauge: percent }
            }

            if is_open() {
                div { class: "flex flex-col w-full justify-start items-start gap-14",
                    CountBox { label: tr.followers, count: followers }
                    CountBox { label: tr.replies, count: replies }
                    CountBox { label: tr.posts, count: posts }
                    CountBox { label: tr.spaces, count: spaces }
                    CountBox { label: tr.votes, count: votes }
                    CountBox { label: tr.surveys, count: surveys }
                }
            }
        }
    }
}

#[component]
pub fn CountBox(label: String, count: i64) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center",
            div { class: "font-medium text-xs/14 text-neutral-400", {label} }
            div { class: "font-extrabold text-sm/15 text-white",
                {count.to_formatted_string(&Locale::en)}
            }
        }
    }
}

#[component]
pub fn GaugeBar(gauge: f32) -> Element {
    let mut percent = use_signal(|| 0.0);

    use_effect(move || {
        percent.set(gauge);
    });

    rsx! {
        div { class: "w-full bg-neutral-800 rounded-full h-6 overflow-hidden",
            div {
                class: "bg-btn-p h-6 rounded-full transition-all duration-700 ease-in-out",
                style: format!("width: {}%;", percent()),
            }
        }
    }
}

#[component]
pub fn Profile(
    lang: Language,
    profile: String,
    description: String,
    clicked: bool,
    onchange_clicked: EventHandler<bool>,
    edit_profile: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: "cursor-pointer flex flex-col w-full justify-start items-start gap-20",
            onclick: move |_| {
                onchange_clicked.call(!clicked);
            },
            div { class: "relative w-fit h-fit",
                if profile != "" {
                    img {
                        class: "w-80 h-80 rounded-full object-cover",
                        src: profile,
                    }
                } else {
                    div { class: "w-80 h-80 rounded-full bg-neutral-400" }
                }
                div { class: "absolute bottom-0 right-0", Grade {} }
            }

            div { class: "flex flex-col w-full justify-start items-start gap-4",
                article {
                    class: "my-profile",
                    dangerous_inner_html: description.clone(),
                }
            }

            div {
                class: "cursor-pointer w-full h-fit",
                onclick: move |e| {
                    edit_profile.call(e);
                },
                EditButton { lang }
            }

        // div { class: "flex flex-row w-full justify-start items-center gap-4",
        //     US {}
        //     div { class: "font-medium text-sm/14 text-[#f9fafb]", "Oregon, United State" }
        // }
        }
    }
}

#[component]
pub fn EditButton(lang: Language) -> Element {
    let tr: EditButtonTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-row w-full justify-center items-center py-15 bg-primary rounded-[10px] font-bold text-[#000203] text-sm/19",
            {tr.edit_profile}
        }
    }
}

translate! {
    EditButtonTranslate;

    edit_profile: {
        ko: "Edit Profile",
        en: "Edit Profile"
    }
}

translate! {
    AccountTranslate;

    switch_account: {
        ko: "Switch Account",
        en: "Switch Account"
    },
    add_another_account: {
        ko: "Add Another Account",
        en: "Add Another Account"
    },
    sign_out: {
        ko: "Sign Out",
        en: "Sign Out"
    }
}

translate! {
    TierTranslate;

    tier: {
        ko: "Tier",
        en: "Tier"
    },
    diamond: {
        ko: "Diamond",
        en: "Diamond"
    }

    followers: {
        ko: "Followers",
        en: "Followers"
    },
    replies: {
        ko: "Replies",
        en: "Replies"
    },
    posts: {
        ko: "Posts",
        en: "Posts"
    },
    spaces: {
        ko: "Spaces",
        en: "Spaces"
    },
    votes: {
        ko: "Votes",
        en: "Votes"
    },
    surveys: {
        ko: "Surveys",
        en: "Surveys"
    }
}
