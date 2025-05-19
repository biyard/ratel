use bdk::prelude::{
    by_components::icons::arrows::{ShapeArrowDown, ShapeArrowUp},
    *,
};

use crate::{
    components::icons::{Badge, Grade, US},
    pages::components::SideRoundedBox,
};

#[component]
pub fn LeftSideProfile(
    lang: Language,
    name: String,
    profile: String,
    description: String,
    exp: i64,
    total_exp: i64,
) -> Element {
    let mut is_clicked = use_signal(|| true);

    rsx! {
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
                            Profile { profile, description }
                            Tier { lang, exp, total_exp }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Tier(lang: Language, exp: i64, total_exp: i64) -> Element {
    let tr: TierTranslate = translate(&lang);

    let percent = if total_exp > 0 {
        (exp as f32 / total_exp as f32) * 100.0
    } else {
        0.0
    };

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-10",
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
pub fn Profile(profile: String, description: String) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-20",
            div { class: "relative w-fit h-fit",
                img {
                    class: "w-80 h-80 rounded-full object-cover",
                    src: profile,
                }
                div { class: "absolute bottom-0 right-0", Grade {} }
            }

            div { class: "flex flex-col w-full justify-start items-start gap-4",
                div { class: "font-medium text-sm/14 text-[#f9fafb]", {description} }
            }

            div { class: "flex flex-row w-full justify-start items-center gap-4",
                US {}
                div { class: "font-medium text-sm/14 text-[#f9fafb]", "Oregon, United State" }
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
