use crate::spaces::space_common::providers::use_space_context;

use super::*;

#[component]
pub fn Administrators() -> Element {
    let tr: GeneralTranslate = use_translate();
    let ctx = use_space_context();
    let space = use_space();

    let administrator = use_loader(move || async move {
        let space_id = space().id;
        if ctx.role().is_admin() {
            get_space_administrator(space_id).await.map(Some)
        } else {
            Ok(None)
        }
    })?;

    rsx! {
        Card {
            div { class: "flex justify-between items-center self-stretch py-4 px-5 border-b border-separator",
                p { class: "font-bold font-raleway text-[24px]/[28px] tracking-[-0.24px] text-web-font-primary",
                    {tr.administrator}
                }
            }

            div { class: "flex flex-col items-start self-stretch p-5 gap-[10px] bg-card max-mobile:p-4",
                if let Some(admin) = administrator() {
                    AdministratorRow {
                        name: admin.display_name,
                        username: admin.username,
                        profile_url: admin.profile_url,
                    }
                } else {
                    p { class: "font-medium leading-6 font-raleway text-[15px] tracking-[0.5px] text-card-meta",
                        {tr.administrator_not_found}
                    }
                }
            }
        }
    }
}

#[component]
fn AdministratorRow(name: String, username: String, profile_url: String) -> Element {
    let profile = if profile_url.trim().is_empty() {
        DEFAULT_PROFILE_IMAGE.to_string()
    } else {
        profile_url
    };

    rsx! {
        div { class: "flex justify-between items-center py-3 px-4 w-full border rounded-[12px] border-separator bg-card max-tablet:flex-col max-tablet:items-start max-tablet:gap-3",
            div { class: "flex items-center gap-[10px]",
                img {
                    src: "{profile}",
                    alt: "{name}",
                    class: "object-cover w-12 h-12 rounded-full shrink-0",
                }

                div { class: "flex flex-col gap-1 items-start",
                    div { class: "flex gap-1 items-center",
                        p { class: "font-bold leading-5 font-raleway text-[17px] tracking-[-0.18px] text-web-font-primary",
                            "{name}"
                        }
                    }
                    p { class: "font-semibold leading-4 font-raleway text-[13px] tracking-[-0.14px] text-web-font-neutral",
                        "@{username}"
                    }
                }
            }
        }
    }
}
