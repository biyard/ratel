use bdk::prelude::*;
use dto::by_components::rich_texts::RichText;

#[component]
pub fn EditProfilePopup(
    lang: Language,
    profile: String,
    nickname: String,
    description: String,
    onedit: EventHandler<(String, String, String)>,
) -> Element {
    let tr: EditProfilePopupTranslate = translate(&lang);

    let profile = use_signal(|| profile);
    let mut nickname = use_signal(|| nickname);
    let mut description = use_signal(|| description);

    rsx! {
        div { class: "w-full max-w-400 mx-5 max-mobile:!max-w-full",
            div { class: "flex flex-col w-400 max-mobile:!w-full gap-20",
                div { class: "flex flex-row w-full justify-center items-center",
                    if !profile().is_empty() {
                        img {
                            class: "w-50 h-50 object-cover rounded-full",
                            src: profile(),
                        }
                    } else {
                        div { class: "w-50 h-50 rounded-full bg-neutral-400" }
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    div { class: "font-semibold text-white text-sm/16", {tr.nickname} }
                    input {
                        class: "bg-black text-neutral-400 placeholder-neutral-600 focus:outline-none w-full font-medium text-sm/16 p-10 rounded-lg",
                        r#type: "text",
                        placeholder: tr.input_nickname_hint,
                        value: nickname(),
                        onchange: move |e| {
                            nickname.set(e.value());
                        },
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start gap-10",
                    div { class: "font-semibold text-white text-sm/16", {tr.description} }
                    div { class: "rounded-lg w-full h-fit justify-start items-start border border-neutral-600",
                        RichText {
                            id: "edit profile description",
                            content: description(),
                            onchange: move |v| {
                                description.set(v);
                            },
                            change_location: true,
                            remove_border: true,
                            placeholder: tr.input_description_hint,
                        }
                    }
                }

                div {
                    class: "cursor-pointer flex flex-row w-full justify-center items-center py-15 bg-primary rounded-[10px] font-bold text-[#000203] text-sm/19",
                    onclick: move |_| {
                        onedit.call((profile(), nickname(), description()));
                    },
                    {tr.edit_profile}
                }
            }
        }
    }
}

translate! {
    EditProfilePopupTranslate;

    nickname: {
        ko: "Nickname",
        en: "Nickname"
    }
    input_nickname_hint: {
        ko: "Input nickname",
        en: "Input nickname"
    }
    description: {
        ko: "Description",
        en: "Description"
    }
    input_description_hint: {
        ko: "Input description",
        en: "Input description"
    }
    edit_profile: {
        ko: "Edit Profile",
        en: "Edit Profile"
    }
}
