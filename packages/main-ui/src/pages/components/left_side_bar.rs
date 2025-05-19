use bdk::prelude::{
    by_components::icons::{
        file::File, other_devices::Bookmark, time::Update, user::User, validations::Add,
    },
    *,
};

use crate::{
    components::icons::{Palace, Pentagon2, RewardCoin},
    pages::components::{LeftSideProfile, SideRoundedAccordian, SideRoundedBox},
};

#[component]
pub fn LeftSidebar(lang: Language) -> Element {
    let tr: LeftSidebarTranslate = translate(&lang);
    //FIXME: fix to connect api
    let recent = vec![
        "Crypto/DAO Treasury Transparent".to_string(),
        "Crypto/DAO Act Investor".to_string(),
        "Crypto/DAO Welcome to Protector".to_string(),
    ];

    let spaces = vec![
        "Crypto/DAO Treasury Transparent".to_string(),
        "Crypto/DAO Act Investor".to_string(),
        "Crypto/DAO Welcome to Protector".to_string(),
    ];

    let communities = vec![
        "Crypto/DAO Treasury Transparent".to_string(),
        "Crypto/DAO Act Investor".to_string(),
        "Crypto/DAO Welcome to Protector".to_string(),
    ];

    rsx! {
        //FIXME: fix to connect api
        div { class: "flex flex-col w-fit h-fit gap-10 justify-start items-start",
            LeftSideProfile {
                lang,
                name: "Hyejin Choi",
                profile: "https://lh3.googleusercontent.com/a/ACg8ocIGf0gpB8MQdGkp5TXW1327nRpuPz70iy_hQY2NXNwanRXbFw=s96-c",
                description: "Office of Rep.",
                exp: 4,
                total_exp: 5,
            }

            SideRoundedBox {
                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    LeftSideItem {
                        icon: rsx! {
                            File { class: "[&>path]:stroke-neutral-500", width: "20", height: "20" }
                        },
                        text: tr.my_posts,
                    }
                    LeftSideItem {
                        icon: rsx! {
                            User { class: "[&>path]:stroke-neutral-500", width: "20", height: "20" }
                        },
                        text: tr.my_profile,
                    }
                    LeftSideItem {
                        icon: rsx! {
                            Bookmark { class: "[&>path]:stroke-neutral-500", width: "20", height: "20" }
                        },
                        text: tr.saved_feeds,
                    }
                    LeftSideItem {
                        icon: rsx! {
                            RewardCoin { class: "[&>path]:stroke-neutral-500", width: "20", height: "20" }
                        },
                        text: tr.sponsoring,
                    }
                }
            }
            SideRoundedAccordian {
                icon: rsx! {
                    Update { class: "[&>path]:stroke-neutral-500", width: "20", height: "20" }
                },
                title: tr.recent,

                ContentList { contents: recent }
            }
            SideRoundedAccordian {
                icon: rsx! {
                    Palace { class: "[&>path]:stroke-neutral-500", width: "20", height: "20" }
                },
                title: tr.spaces,
                div { class: "flex flex-col w-full justify-start items-start gap-16",
                    button {
                        class: "cursor-pointer flex flex-row w-full justify-start items-center gap-4",
                        onclick: move |_| {
                            tracing::debug!("create a spaces button clicked");
                        },
                        Add {
                            class: "[&>path]:stroke-white",
                            width: "20",
                            height: "20",
                        }
                        div { class: "font-bold text-white text-sm/16", {tr.create_space} }
                    }
                    ContentList { contents: spaces }
                }
            
            }
            SideRoundedAccordian {
                icon: rsx! {
                    Pentagon2 {
                        class: "[&>path]:stroke-neutral-500 [&>path]:fill-transparent",
                        width: "20",
                        height: "20",
                    }
                },
                title: tr.communities,
                div { class: "flex flex-col w-full justify-start items-start gap-16",
                    button {
                        class: "cursor-pointer flex flex-row w-full justify-start items-center gap-4",
                        onclick: move |_| {
                            tracing::debug!("create a community button clicked");
                        },
                        Add {
                            class: "[&>path]:stroke-white",
                            width: "20",
                            height: "20",
                        }
                        div { class: "font-bold text-white text-sm/16", {tr.create_community} }
                    }
                    ContentList { contents: communities }
                }
            
            }
        }
    }
}

#[component]
pub fn ContentList(contents: Vec<String>) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-16",
            for content in contents.iter().take(3) {
                button { class: "cursor-pointer w-full justify-start items-start font-normal text-white text-base/16 overflow-hidden text-ellipsis whitespace-nowrap text-start",
                    {content.clone()}
                }
            }
        }
    }
}

#[component]
pub fn LeftSideItem(icon: Element, text: String) -> Element {
    rsx! {
        button { class: "cursor-pointer flex flex-row w-full justify-start items-center gap-4",
            {icon}
            div { class: "font-bold text-white text-sm/16", {text} }
        }
    }
}

translate! {
    LeftSidebarTranslate;

    my_posts: {
        ko: "My Posts",
        en: "My Posts"
    },
    my_profile: {
        ko: "My Profile",
        en: "My Profile"
    },
    saved_feeds: {
        ko: "Saved Feeds",
        en: "Saved Feeds"
    },
    sponsoring: {
        ko: "Sponsoring",
        en: "Sponsoring"
    },

    recent: {
        ko: "Recent",
        en: "Recent"
    },
    spaces: {
        ko: "Spaces",
        en: "Spaces"
    },
    communities: {
        ko: "Communities",
        en: "Communities"
    }

    create_space: {
        ko: "Create a Space",
        en: "Create a Space"
    },
    create_community: {
        ko: "Create a Community",
        en: "Create a Community"
    }
}
