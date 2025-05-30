use bdk::prelude::{
    by_components::icons::{
        arrows::{BendArrowRight, ChevronDown, ChevronUp},
        chat::SquareChat,
        emoji::ThumbsUp,
        validations::Extra,
    },
    *,
};
use num_format::{Locale, ToFormattedString};

use crate::{
    dto::comment::Comment, pages::components::CreateReplyBox, utils::time::format_prev_time,
};

#[component]
pub fn ThreadComments(lang: Language, number_of_comments: i64, comments: Vec<Comment>) -> Element {
    let tr: ThreadCommentsTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-20",
            div { class: "flex flex-row w-full justify-start items-start py-20 gap-8",
                SquareChat {
                    fill: "none",
                    class: "[&>g>path]:stroke-white",
                    width: "24",
                    height: "24",
                }
                div { class: "font-medium text-white text-base/24",
                    {format!("{} {}", number_of_comments.to_formatted_string(&Locale::en), tr.reply)}
                }
            }

            for (i , comment) in comments.iter().enumerate() {
                ThreadComment { lang, comment: comment.clone() }

                if i != comments.len() - 1 {
                    LineContainer {}
                }
            }
        }
    }
}

#[component]
pub fn LineContainer() -> Element {
    rsx! {
        div { class: "flex flex-row w-full h-1 bg-neutral-800" }
    }
}

#[component]
pub fn ThreadComment(lang: Language, comment: Comment) -> Element {
    let mut reply_clicked = use_signal(|| false);

    rsx! {
        div { class: "flex flex-row w-full justify-start items-center",
            div { class: "flex flex-row w-fit justify-start items-start gap-8",
                img {
                    class: "w-40 h-40 rounded-full object-cover",
                    src: comment.profile_url,
                }
                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    div { class: "flex flex-row w-full justify-between items-center",
                        div { class: "flex flex-col w-fit justify-start items-start gap-2",
                            div { class: "font-semibold text-neutral-300 text-[15px]/18",
                                {comment.profile_name}
                            }
                            div { class: "font-semibold text-neutral-400 text-xs/20",
                                {format_prev_time(comment.created_at)}
                            }
                        }

                        Extra {
                            class: "[&>circle]:fill-neutral-500",
                            width: "24",
                            height: "24",
                        }
                    }

                    CommentDescription { lang, comment: comment.comment }

                    CommentBottom {
                        lang,
                        number_of_comments: comment.number_of_comments,
                        number_of_likes: comment.number_of_likes,

                        is_clicked: reply_clicked(),
                        on_clicked: move |_| {
                            reply_clicked.set(!reply_clicked());
                        },
                    }

                    CreateReplyBox {
                        id: format!("comment {}", comment.id),
                        lang,
                        onsend: move |_| {},
                    }

                    div { class: "flex flex-col w-full justify-start items-start mt-5 gap-25 mb-6",
                        for reply in comment.replies.clone() {
                            ReplyComment { lang, reply }
                        }
                    }

                    div {
                        class: "cursor-pointer w-fit h-fit p-4 bg-primary rounded-lg aria-hidden:!hidden",
                        onclick: move |_| {
                            reply_clicked.set(false);
                        },
                        aria_hidden: comment.replies.is_empty() || !reply_clicked(),
                        ChevronUp {
                            class: "[&>path]:stroke-[#18181B]",
                            width: "24",
                            height: "24",
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ReplyComment(lang: Language, reply: Comment) -> Element {
    rsx! {
        div { class: "flex flex-row w-fit justify-start items-start gap-8",
            img {
                class: "w-32 h-32 rounded-full object-cover",
                src: reply.profile_url,
            }
            div { class: "flex flex-col w-full justify-start items-start gap-20",
                div { class: "flex flex-row w-full justify-between items-center",
                    div { class: "flex flex-col w-fit justify-start items-start gap-2",
                        div { class: "font-semibold text-neutral-300 text-[14px]/16",
                            {reply.profile_name}
                        }
                        div { class: "font-semibold text-neutral-400 text-xs/20",
                            {format_prev_time(reply.created_at)}
                        }
                    }
                }

                CommentDescription { lang, comment: reply.comment }
            }
        }
    }
}

#[component]
pub fn CommentBottom(
    lang: Language,
    number_of_comments: i64,
    number_of_likes: i64,
    is_clicked: bool,
    on_clicked: EventHandler<MouseEvent>,
) -> Element {
    let mut is_hover = use_signal(|| false);
    let tr: ThreadCommentsTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-row w-full justify-between item-center",
            div { class: "flex flex-row w-fit justify-between items-center gap-40",
                div {
                    class: "cursor-pointer flex flex-row w-fit justify-start items-center gap-8 bg-transparent aria-selected:!bg-primary px-8 py-4 rounded-lg hover:!bg-primary",
                    aria_selected: is_clicked,
                    onclick: move |e| {
                        on_clicked.call(e);
                    },
                    onmouseenter: move |_| {
                        is_hover.set(true);
                    },
                    onmouseleave: move |_| {
                        is_hover.set(false);
                    },
                    div {
                        class: "font-medium text-base/24 text-primary aria-selected:text-neutral-900",
                        aria_selected: is_clicked || is_hover(),
                        {
                            format!(
                                "{} {}",
                                { number_of_comments.to_formatted_string(&Locale::en) },
                                tr.reply,
                            )
                        }
                    }

                    if is_clicked {
                        ChevronUp {
                            class: format!("[&>path]:stroke-neutral-900"),
                            width: "24",
                            height: "24",
                        }
                    } else {
                        ChevronDown {
                            class: format!(
                                "{}",
                                if is_hover() {
                                    "[&>path]:stroke-neutral-900"
                                } else {
                                    "[&>path]:stroke-primary"
                                },
                            ),
                            width: "24",
                            height: "24",
                        }
                    }
                }

                div { class: "cursor-pointer flex flex-row w-fit justify-start items-center gap-8",
                    BendArrowRight {
                        class: "[&>path]:stroke-white",
                        width: "24",
                        height: "24",
                    }
                    div { class: "font-medium text-neutral-300 text-base/24", {tr.reply} }
                }
            }

            div { class: "flex flex-row w-fit justify-between items-center gap-40",
                div { class: "cursor-pointer flex flex-row w-fit justify-start items-center gap-8",
                    SquareChat {
                        class: "[&>g>path]:stroke-cb-text [&>line]:stroke-cb-text",
                        width: "24",
                        height: "24",
                        fill: "none",
                    }
                    div { class: "font-medium text-cb-text text-base/24",
                        {number_of_comments.to_formatted_string(&Locale::en)}
                    }
                }

                div { class: "cursor-pointer flex flex-row w-fit justify-start items-center gap-8",
                    ThumbsUp {
                        class: "[&>path]:stroke-cb-text",
                        width: "24",
                        height: "24",
                    }
                    div { class: "font-medium text-cb-text text-base/24",
                        {number_of_likes.to_formatted_string(&Locale::en)}
                    }
                }
            }
        }
    }
}

#[component]
pub fn CommentDescription(lang: Language, comment: String) -> Element {
    rsx! {
        div {
            class: "w-full font-normal text-[15px]/22 text-neutral-300",
            dangerous_inner_html: comment,
        }
    }
}

translate! {
    ThreadCommentsTranslate;

    reply: {
        ko: "Reply",
        en: "Reply"
    }
}
