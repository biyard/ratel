use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

#[component]
pub fn AdvocacyCampaignsByIdPage(
    id: ReadOnlySignal<i64>,
    #[props(default = Language::En)] lang: Language,
) -> Element {
    let mut ctrl = Controller::new(lang, id)?;
    let tr: AdvocacyCampaingsByIdTranslate = translate(&lang);
    let campaign = ctrl.campaign()?;
    let author = match campaign.author.first() {
        Some(author) => author,
        None => {
            tracing::warn!("No author found for the campaign with id: {}", ctrl.id());
            return rsx! {
                div { "No author information available." }
            };
        }
    };

    rsx! {
        by_components::meta::MetaPage { title: campaign.title.clone() }

        div { class: "absolute bottom-30 flex items-center justify-center w-full z-100",
            button {
                class: "group max-w-desktop w-full btn primary flex items-center justify-center aria-voted:!bg-secondary/30 aria-voted:!text-secondary",
                "aria-voted": campaign.voted,
                onclick: move |_| async move { ctrl.handle_agree().await },
                span { class: "group-aria-voted:!hidden", {tr.btn_agree} }
                span { class: "hidden group-aria-voted:!block", {tr.btn_voted} }
            }
        }

        div {
            id: "advocacy-campaings-by-id",
            class: "relative max-w-desktop min-h-screen flex flex-col gap-50 px-20 pb-100 pt-80 justify-start items-center",
            h1 { class: "heading1", {campaign.title} }

            div {
                id: "advocacy-campaign-content",
                class: "grid grid-cols-3 gap-10 w-full items-start",

                article {
                    id: "advocacy-campaign-details",
                    class: "col-span-2 order-1 max-tablet:order-2 grow",
                    dangerous_inner_html: campaign.contents,
                }

                div { class: "col-span-1 order-2 max-tablet:order-1 flex flex-col items-start gap-20",
                    div { class: "w-full bg-component-bg p-20 flex flex-col gap-30 items-start rounded-lg",
                        h2 { class: "text-xl font-bold", {tr.author} }
                        div { class: "flex flex-col gap-10",
                            div { class: "w-full flex flex-row gap-10 items-center",
                                img {
                                    class: "w-50 object-cover rounded-lg",
                                    src: author.profile_url.clone(),
                                }
                                p { class: "font-bold", {author.nickname.clone()} }
                            }
                            p {
                                class: "flex flex-col text-c-wg-70",
                                dangerous_inner_html: author.html_contents.clone(),
                            }
                        }
                    }
                    p { class: " bg-component-bg rounded-lg p-20 w-full",
                        "지금까지 {campaign.votes}명이 이 법안을 지지하였습니다."
                    }
                }
            }

        } // end of this page
    }
}
