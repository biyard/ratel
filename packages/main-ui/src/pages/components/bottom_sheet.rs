use bdk::prelude::{
    by_components::icons::{
        arrows::{ShapeArrowDown, ShapeArrowUp},
        security::Logout,
        time::Update,
        validations::{Add, Clear},
    },
    *,
};
use wasm_bindgen::{JsCast, prelude::Closure};

use crate::{
    components::icons::{Badge, Grade, Palace, Pentagon2},
    pages::controller::{AccountList, Profile},
};

use web_sys::{TouchEvent, window};

#[component]
pub fn BottomSheet(
    lang: Language,
    profile: Profile,
    recent_feeds: Vec<String>,
    recent_spaces: Vec<String>,
    recent_communities: Vec<String>,

    accounts: Vec<AccountList>,
    add_account: EventHandler<MouseEvent>,
    sign_out: EventHandler<MouseEvent>,
) -> Element {
    let mut profile_clicked = use_signal(|| false);
    let mut is_dragging = use_signal(|| false);
    let mut start_y = use_signal(|| 0.0);
    let mut translate_y = use_signal(|| 70.0);

    use_effect({
        move || {
            let window = window().unwrap();
            let document = window.document().unwrap();
            let body = document.body().unwrap();

            let touch_move = Closure::wrap(Box::new({
                move |event: web_sys::Event| {
                    if !is_dragging() {
                        return;
                    }
                    let event: &TouchEvent = match event.dyn_ref::<TouchEvent>() {
                        Some(e) => e,
                        None => return,
                    };
                    if let Some(touch) = event.touches().item(0) {
                        let y = touch.client_y() as f64;
                        let delta = y - start_y();
                        let height = window.inner_height().unwrap().as_f64().unwrap();
                        let new_val = (translate_y() + delta / height * 100.0).clamp(0.0, 90.0);
                        translate_y.set(new_val);
                        start_y.set(y);
                    }
                }
            }) as Box<dyn FnMut(web_sys::Event)>);

            let touch_end = Closure::wrap(Box::new({
                move |event: web_sys::Event| {
                    is_dragging.set(false);
                    let _: &TouchEvent = match event.dyn_ref::<TouchEvent>() {
                        Some(e) => e,
                        None => return,
                    };

                    let current = translate_y();
                    if current < 30.0 {
                        translate_y.set(0.0);
                    } else if current > 60.0 {
                        translate_y.set(70.0);
                    } else {
                        translate_y.set(current);
                    }
                }
            }) as Box<dyn FnMut(web_sys::Event)>);

            body.add_event_listener_with_callback("touchmove", touch_move.as_ref().unchecked_ref())
                .unwrap();
            body.add_event_listener_with_callback("touchend", touch_end.as_ref().unchecked_ref())
                .unwrap();
            touch_move.forget();
            touch_end.forget();
        }
    });

    tracing::debug!("translate-y: {:?}", translate_y());

    rsx! {
        if translate_y() == 0.0 || profile_clicked() {
            div { class: "fixed inset-0 z-40 bg-black opacity-50" }
        }
        div {
            class: "fixed bottom-0 left-0 w-full z-51 aria-hidden:hidden",
            aria_hidden: !profile_clicked(),
            Account {
                lang,
                nickname: profile.nickname.clone(),
                email: profile.email,
                accounts,
                onprev: move |_| {
                    profile_clicked.set(false);
                },
                add_account,
                sign_out,
            }
        }
        div {
            class: "fixed bottom-100 left-0 w-full z-50 bg-neutral-800 rounded-t-[20px] transition-all duration-300 ease-in-out",
            style: format!(
                "transform: translateY({}%); max-height: 90vh; overflow-y: auto;",
                translate_y(),
            ),

            div {
                class: "flex flex-col w-full justify-start items-start gap-8",
                ontouchstart: move |e| {
                    if let Some(ev) = e.data.downcast::<web_sys::TouchEvent>() {
                        if let Some(touch) = ev.touches().item(0) {
                            tracing::debug!("drag start {:?}", touch.client_y());
                            is_dragging.set(true);
                            start_y.set(touch.client_y() as f64);
                        }
                    }
                },

                div { class: "flex flex-col w-full justify-start items-start pt-5 pb-12 px-20",
                    div { class: "flex flex-row w-full justify-center items-center cursor-pointer",
                        div { class: "w-36 h-5 bg-neutral-600 rounded-lg" }
                    }
                    div { class: "flex flex-row w-full justify-start items-center gap-4",
                        img {
                            class: "w-24 h-24 rounded-full object-cover",
                            src: profile.profile.clone(),
                        }
                        div { class: "font-bold text-lg/21 text-white", {profile.nickname.clone()} }
                        Badge {}
                    }
                }

                if translate_y() < 20.0 {
                    BottomInformation {
                        lang,
                        description: profile.description.clone().unwrap_or_default(),
                        exp: profile.exp,
                        total_exp: profile.total_exp,
                        recent_feeds,
                        recent_communities,
                        recent_spaces,

                        onclick_profile: move |_| {
                            profile_clicked.set(true);
                            is_dragging.set(false);
                            translate_y.set(70.0);
                        },
                    }
                }
            }
        }
    }
}

#[component]
pub fn Account(
    lang: Language,
    nickname: String,
    email: String,
    accounts: Vec<AccountList>,
    onprev: EventHandler<MouseEvent>,
    add_account: EventHandler<MouseEvent>,
    sign_out: EventHandler<MouseEvent>,
) -> Element {
    let tr: AccountTranslate = translate(&lang);
    let accounts: Vec<AccountList> = vec![];
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start px-20 pt-20 pb-30 bg-neutral-900 rounded-t-[20px] border-t border-t-neutral-700 gap-20",
            div { class: "flex flex-row w-full justify-between items-center",
                div { class: "flex flex-col w-fit justify-start items-start gap-4",
                    div { class: "font-bold text-lg/20 text-white", {nickname} }
                    div { class: "font-semibold text-sm/20 text-neutral-500", {email} }
                }

                div {
                    class: "cursor-pointer w-fit h-fit p-7 bg-[#27272a] rounded-full",
                    onclick: move |e| {
                        onprev.call(e);
                    },
                    Clear {
                        class: "[&>path]:stroke-neutral-400",
                        width: "15",
                        height: "15",
                    }
                }
            }

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

                DivideContainer {}

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
}

#[component]
pub fn BottomInformation(
    lang: Language,
    description: String,
    exp: i64,
    total_exp: i64,
    recent_feeds: Vec<String>,
    recent_spaces: Vec<String>,
    recent_communities: Vec<String>,

    onclick_profile: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start bg-neutral-900 p-20",
            div { class: "flex flex-col w-full justify-start items-start gap-10",
                div {
                    class: "cursor-pointer flex flex-col w-full justify-start items-start gap-4",
                    onclick: move |e| {
                        onclick_profile.call(e);
                    },
                    div { class: "font-medium text-sm/14 text-[#f9fafb]", {description} }
                }

                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    Tier { lang, exp, total_exp }
                    div { class: "flex flex-col w-full justify-start items-start",
                        Content {
                            icon: rsx! {
                                Update { class: "[&>path]:stroke-neutral-500", width: "20", height: "20" }
                            },
                            title: "Recent",
                            contents: recent_feeds,
                        }
                        DivideContainer {}
                        Content {
                            icon: rsx! {
                                Palace { class: "[&>path]:stroke-neutral-500", width: "20", height: "20" }
                            },
                            title: "Spaces",
                            contents: recent_spaces,
                        }
                        DivideContainer {}
                        Content {
                            icon: rsx! {
                                Pentagon2 {
                                    class: "[&>path]:stroke-neutral-500 [&>path]:fill-transparent",
                                    width: "20",
                                    height: "20",
                                }
                            },
                            title: "Communities",
                            contents: recent_communities,
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn DivideContainer() -> Element {
    rsx! {
        div { class: "flex flex-row w-full h-1 bg-neutral-800" }
    }
}

#[component]
pub fn Tier(lang: Language, exp: i64, total_exp: i64) -> Element {
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
        }
    }
}

#[component]
pub fn Content(icon: Element, title: String, contents: Vec<String>) -> Element {
    let mut is_clicked = use_signal(|| true);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start px-16 py-20 gap-20",
            div {
                class: "cursor-pointer flex flex-row w-full justify-between items-center",
                onclick: move |_| {
                    is_clicked.set(!is_clicked());
                },
                div { class: "flex flex-row w-fit justify-start items-center gap-4",
                    {icon}
                    div { class: "font-bold text-sm/16 text-neutral-500", {title} }
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
                div { class: "flex flex-col w-full justify-start items-start gap-16",
                    for content in contents.iter().take(3) {
                        button { class: "cursor-pointer w-full justify-start items-start font-normal text-white text-base/16 overflow-hidden text-ellipsis whitespace-nowrap text-start",
                            {content.clone()}
                        }
                    }
                }
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
