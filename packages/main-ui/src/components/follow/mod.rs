use bdk::prelude::{by_components::icons::validations::Add, *};

#[component]
pub fn Follow(lang: Language, onclick: EventHandler<MouseEvent>) -> Element {
    let tr: FollowTranslate = translate(&lang);

    rsx! {
        div {
            class: "cursor-pointer flex flex-row justify-start items-center px-10 py-5 bg-white rounded-[50px] gap-3",
            onclick,
            Add { width: "15", height: "15" }
            div { class: "font-medium text-[#000203] text-xs/14", {tr.follow} }
        }
    }
}

translate! {
    FollowTranslate;

    follow: {
        ko: "Follow",
        en: "Follow"
    }
}
