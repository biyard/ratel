use crate::common::components::{
    Badge, BadgeColor, BadgeSize, BadgeVariant, Collapsible, CollapsibleContent,
    CollapsibleTrigger,
};
use crate::common::utils::time::time_ago;
use crate::features::spaces::pages::actions::actions::discussion::views::main::viewer::DiscussionViewerTranslate;
use crate::features::spaces::pages::actions::actions::discussion::*;
use dioxus_translate::*;
use lucide_dioxus::{Calendar, Clock, MessageCircle, Tag, User, Zap};
use time::{OffsetDateTime, format_description};

fn format_period(start_ms: i64, end_ms: i64) -> String {
    let fmt = match format_description::parse("[month repr:short] [day padding:none], [year]") {
        Ok(f) => f,
        Err(_) => return String::new(),
    };
    let start = OffsetDateTime::from_unix_timestamp(start_ms / 1000)
        .unwrap_or(OffsetDateTime::UNIX_EPOCH);
    let end = OffsetDateTime::from_unix_timestamp(end_ms / 1000)
        .unwrap_or(OffsetDateTime::UNIX_EPOCH);
    let start_s = start.format(&fmt).unwrap_or_default();
    let end_s = end.format(&fmt).unwrap_or_default();
    format!("{start_s} – {end_s}")
}

fn status_color(status: &DiscussionStatus) -> BadgeColor {
    match status {
        DiscussionStatus::NotStarted => BadgeColor::Grey,
        DiscussionStatus::InProgress => BadgeColor::Green,
        DiscussionStatus::Finish => BadgeColor::Blue,
    }
}

#[component]
pub fn DiscussionProperties(discussion: SpacePost) -> Element {
    let tr: DiscussionViewerTranslate = use_translate();

    let status = discussion.status();
    let status_label = match status {
        DiscussionStatus::NotStarted => tr.status_not_started,
        DiscussionStatus::InProgress => tr.status_in_progress,
        DiscussionStatus::Finish => tr.status_finish,
    };
    let status_badge_color = status_color(&status);
    let period = format_period(discussion.started_at, discussion.ended_at);
    let created = time_ago(discussion.created_at);
    let category = if discussion.category_name.is_empty() {
        "—".to_string()
    } else {
        discussion.category_name.clone()
    };
    let comments = discussion.comments.max(0);

    rsx! {
        Collapsible {
            div { class: "rounded-lg border border-border bg-card-bg",
                CollapsibleTrigger {
                    div { class: "flex gap-2 items-center px-4 py-2 text-sm font-medium text-foreground-muted",
                        "{tr.properties}"
                    }
                }
                CollapsibleContent {
                    div { class: "flex flex-col gap-2 px-4 pt-2 pb-4 text-sm",

                        PropertyRow {
                            label: tr.status.to_string(),
                            icon: rsx! {
                                Zap { class: "w-4 h-4 [&>path]:stroke-icon-primary" }
                            },
                            value: rsx! {
                                Badge {
                                    color: status_badge_color,
                                    size: BadgeSize::Small,
                                    variant: BadgeVariant::Rounded,
                                    "{status_label}"
                                }
                            },
                        }

                        PropertyRow {
                            label: tr.author.to_string(),
                            icon: rsx! {
                                User { class: "w-4 h-4 [&>path]:stroke-icon-primary" }
                            },
                            value: rsx! {
                                div { class: "flex gap-2 items-center",
                                    if !discussion.author_profile_url.is_empty() {
                                        img {
                                            class: "object-cover w-5 h-5 rounded-full",
                                            src: "{discussion.author_profile_url}",
                                            alt: "{discussion.author_display_name}",
                                        }
                                    }
                                    span { class: "text-text-primary", "{discussion.author_display_name}" }
                                }
                            },
                        }

                        PropertyRow {
                            label: tr.category.to_string(),
                            icon: rsx! {
                                Tag { class: "w-4 h-4 [&>path]:stroke-icon-primary" }
                            },
                            value: rsx! {
                                span { class: "text-text-primary", "{category}" }
                            },
                        }

                        PropertyRow {
                            label: tr.period.to_string(),
                            icon: rsx! {
                                Calendar { class: "w-4 h-4 [&>path]:stroke-icon-primary" }
                            },
                            value: rsx! {
                                span { class: "text-text-primary", "{period}" }
                            },
                        }

                        PropertyRow {
                            label: tr.comments.to_string(),
                            icon: rsx! {
                                MessageCircle { class: "w-4 h-4 [&>path]:stroke-icon-primary" }
                            },
                            value: rsx! {
                                span { class: "text-text-primary", "{comments}" }
                            },
                        }

                        PropertyRow {
                            label: tr.created.to_string(),
                            icon: rsx! {
                                Clock { class: "w-4 h-4 [&>path]:stroke-icon-primary" }
                            },
                            value: rsx! {
                                span { class: "text-text-primary", "{created}" }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PropertyRow(label: String, icon: Element, value: Element) -> Element {
    rsx! {
        div { class: "grid grid-cols-[140px_1fr] gap-3 items-center",
            div { class: "flex gap-2 items-center text-foreground-muted",
                {icon}
                span { "{label}" }
            }
            div { {value} }
        }
    }
}
