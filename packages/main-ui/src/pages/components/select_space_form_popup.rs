use bdk::prelude::*;
use dto::{
    SpaceForm,
    by_components::icons::{chat::Discuss, email::Vote, shopping::Cube},
};

use crate::components::{icons::Palace, selectbox::rounded_selectbox::RoundedSelectbox};

#[component]
pub fn SelectSpaceFormPopup(lang: Language, onsend: EventHandler<SpaceForm>) -> Element {
    let tr: SelectSpaceFormPopupTranslate = translate(&lang);
    let mut selected_form = use_signal(|| None);

    rsx! {
        div { class: "w-full max-w-400 mx-5 max-mobile:!max-w-full",
            div { class: "flex flex-col w-400 max-mobile:!w-full gap-10",
                SpaceFormComponent {
                    icon: rsx! {
                        Palace { class: "[&>path]:stroke-neutral-500", width: "32", height: "32" }
                    },
                    label: tr.legislation,
                    description: tr.legislation_description,
                    checked: selected_form() == Some(SpaceForm::Legislation),
                    oncheck: move |_| {
                        selected_form.set(Some(SpaceForm::Legislation));
                    },
                }
                SpaceFormComponent {
                    icon: rsx! {
                        Vote {
                            class: "[&>path]:stroke-neutral-500 [&>rect]:stroke-neutral-500",
                            width: "32",
                            height: "32",
                        }
                    },
                    label: tr.poll,
                    description: tr.poll_description,
                    checked: selected_form() == Some(SpaceForm::Poll),
                    oncheck: move |_| {
                        selected_form.set(Some(SpaceForm::Poll));
                    },
                }
                SpaceFormComponent {
                    icon: rsx! {
                        Discuss {
                            class: "[&>path]:stroke-neutral-500",
                            width: "32",
                            height: "32",
                            fill: "none",
                        }
                    },
                    label: tr.deliberation,
                    description: tr.deliberation_description,
                    checked: selected_form() == Some(SpaceForm::Deliberation),
                    oncheck: move |_| {
                        selected_form.set(Some(SpaceForm::Deliberation));
                    },
                }
                SpaceFormComponent {
                    icon: rsx! {
                        Cube { class: "[&>path]:stroke-neutral-500", width: "32", height: "32" }
                    },
                    label: tr.nft,
                    description: tr.nft_description,
                    checked: selected_form() == Some(SpaceForm::Nft),
                    oncheck: move |_| {
                        selected_form.set(Some(SpaceForm::Nft));
                    },
                }
            }

            SendButton {
                lang,
                enabled: selected_form().is_some(),
                onclick: move |_| {
                    if let Some(form) = selected_form() {
                        onsend.call(form);
                    }
                },
            }
        }
    }
}

#[component]
pub fn SendButton(lang: Language, enabled: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let tr: SendButtonTranslate = translate(&lang);
    rsx! {
        div {
            class: "cursor-pointer aria-disabled:cursor-not-allowed flex flex-col my-25 w-full justify-center items-center py-15 bg-primary aria-disabled:bg-neutral-800 rounded-[10px] font-bold text-base/19 text-[#000203] aria-disabled:text-neutral-700",
            aria_disabled: !enabled,
            onclick,
            {tr.send}
        }
    }
}

#[component]
pub fn SpaceFormComponent(
    icon: Element,
    label: String,
    description: String,
    checked: bool,
    oncheck: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-row w-full h-fit min-h-115 justify-start items-center p-20 gap-10 border border-neutral-800 aria-checked:border-primary rounded-[10px]",
            aria_checked: checked,
            {icon}
            div { class: "flex flex-col flex-1 gap-4",
                div { class: "font-bold text-white text-[15px]/20", {label} }
                div { class: "font-normal text-neutral-300 text-[15px]/24", {description} }
            }

            RoundedSelectbox {
                selected: checked,
                onchange: move |e| {
                    oncheck.call(e);
                },
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
    SelectSpaceFormPopupTranslate;

    legislation: {
        ko: "Legislation",
        en: "Legislation"
    },
    legislation_description: {
        ko: "Propose and decide on new rules or policies.",
        en: "Propose and decide on new rules or policies."
    },

    poll: {
        ko: "Poll",
        en: "Poll"
    },
    poll_description: {
        ko: "Collect quick opinions or preferences.",
        en: "Collect quick opinions or preferences."
    },

    deliberation: {
        ko: "Deliberation",
        en: "Deliberation"
    },
    deliberation_description: {
        ko: "Share perspectives and engage in in-depth discussion.",
        en: "Share perspectives and engage in in-depth discussion."
    },

    nft: {
        ko: "NFT",
        en: "NFT"
    },
    nft_description: {
        ko: "Submit information to issue an NFT.",
        en: "Submit information to issue an NFT."
    }
}
