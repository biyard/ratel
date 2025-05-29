use bdk::prelude::{
    by_components::icons::{edit::Search, validations::Clear},
    *,
};

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
                class: "bg-transparent text-neutral-400 placeholder-neutral-500 focus:outline-none w-full font-medium text-sm/16",
                r#type: "text",
                placeholder: tr.search,
                onchange: move |e| {
                    onsearch.call(e.value());
                },
            }
        }
    }
}

#[component]
pub fn MobileSearchBox(
    lang: Language,
    onsearch: EventHandler<String>,
    onextend: EventHandler<MouseEvent>,
) -> Element {
    let mut value = use_signal(|| "".to_string());
    let tr: SearchBoxTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-row w-full justify-start items-center pl-12 pr-4 py-4 gap-4",
            Search {
                class: "[&>path]:stroke-neutral-500 [&>circle]:stroke-neutral-500",
                width: "24",
                height: "24",
            }

            input {
                class: "bg-transparent text-neutral-400 placeholder-neutral-500 focus:outline-none w-full font-medium text-sm/16",
                r#type: "text",
                placeholder: tr.search,
                value,
                onchange: move |e| {
                    value.set(e.value());
                },
                onkeypress: move |e: KeyboardEvent| {
                    let key = e.key();
                    if key == Key::Enter {
                        onsearch.call(value());
                        value.set("".to_string());
                    }
                },
            }

            div {
                class: "cursor-pointer w-fit h-fit p-7 rounded-full bg-[#27272a]",
                onclick: move |e| {
                    onextend.call(e);
                },
                Clear {
                    class: "[&>path]:stroke-neutral-400",
                    width: "10",
                    height: "10",
                }
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
