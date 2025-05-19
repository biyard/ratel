use bdk::prelude::{by_components::icons::edit::Search, *};

#[component]
pub fn SearchBox(lang: Language, onsearch: EventHandler<String>) -> Element {
    let tr: SearchBoxTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-row min-w-[375px] min-h-[42px] w-full justify-start items-center rounded-[100px] bg-[#171717] px-12 py-9 gap-4",
            Search {
                class: "[&>path]:stroke-[#737373] [&>circle]:stroke-[#737373]",
                width: "24",
                height: "24",
            }
            input {
                class: "bg-transparent text-neutral-400 placeholder-neutral-500 focus:outline-none w-full font-medium text-sm leading-4",
                r#type: "text",
                placeholder: tr.search,
                onchange: move |e| {
                    onsearch.call(e.value());
                },
            }
        }
    }
}

translate! {
    SearchBoxTranslate;

    search: {
        ko: "Search",
        en: "Search"
    }
}
