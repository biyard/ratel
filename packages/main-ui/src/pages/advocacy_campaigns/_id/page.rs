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

        div { class: "absolute fixed bg-background py-30 bottom-0 flex items-center justify-center w-full z-100 px-20",
            button {
                class: "group max-w-desktop w-full btn secondary flex items-center justify-center aria-voted:!bg-neutral-600 aria-voted:!text-secondary",
                "aria-voted": campaign.voted,
                onclick: move |_| async move { ctrl.handle_agree().await },
                span { class: "group-aria-voted:!hidden", {tr.btn_agree} }
                span { class: "hidden group-aria-voted:!block", {tr.btn_voted} }
            }
        }

        div {
            id: "advocacy-campaings-by-id",
            class: "max-w-desktop min-h-screen flex flex-col gap-50 px-20 pb-100 pt-80 justify-start items-center max-tablet:!gap-10",
            h1 { class: "heading1 max-tablet:!text-3xl", {campaign.title} }

            div {
                id: "advocacy-campaign-content",
                class: "grid grid-cols-3 gap-10 w-full items-start max-tablet:!flex max-tablet:!flex-col",

                article {
                    id: "advocacy-campaign-details",
                    class: "col-span-2 order-1 max-tablet:order-2 grow",
                    dangerous_inner_html: campaign.contents,
                }

                div { class: "col-span-1 order-2 max-tablet:order-1 flex flex-col items-start gap-20 max-tablet:!gap-5",
                    div { class: "w-full bg-component-bg p-20 flex flex-col gap-30 items-start rounded-lg max-tablet:!bg-transparent max-tablet:!gap-10 max-tablet:!p-0",
                        h2 { class: "text-xl font-bold max-tablet:!hidden", {tr.author} }
                        div { class: "flex flex-col gap-10",
                            div { class: "w-full flex flex-row gap-10 items-center",
                                img {
                                    class: "w-50 object-cover rounded-lg max-tablet:!w-20",
                                    src: author.profile_url.clone(),
                                }
                                p { class: "font-bold max-tablet:!text-c-wg-50",
                                    {author.nickname.clone()}
                                }
                            }
                            p { class: "flex flex-col text-c-wg-70 max-tablet:!hidden",
                                p { dangerous_inner_html: author.html_contents.clone() }
                            }
                        }
                    }
                    p { class: " bg-component-bg rounded-lg p-20 w-full max-tablet:!bg-transparent max-tablet:!p-0",
                        "지금까지 "
                        span { class: "text-primary font-bold", "{campaign.votes}명" }
                        "이 이 법안을 지지하였습니다."
                    }
                }
            }

        } // end of this page
    }
}
