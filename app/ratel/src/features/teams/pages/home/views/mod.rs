use crate::common::*;
use crate::features::teams::pages::home::components::*;
use crate::features::teams::pages::home::HomeViewMode;
use crate::features::posts::controllers::create_post::create_post_handler;
use crate::features::posts::*;
use dioxus::prelude::*;

translate! {
    HomeTranslate;

    create: {
        en: "Create",
        ko: "작성",
    },
}

#[component]
pub fn Home(teamname: String) -> Element {
    let tr: HomeTranslate = use_translate();
    let mut view_mode: Signal<HomeViewMode> = use_signal(|| HomeViewMode::List);
    let team_ctx = crate::common::contexts::use_team_context();
    let nav = use_navigator();

    let team_item = {
        let teams = team_ctx.teams.read();
        teams.iter().find(|team| team.username == teamname).cloned()
    };

    let team_pk_str = team_item.as_ref().map(|t| t.pk.clone());

    let (display_name, profile_url, description) = match team_item {
        Some(team) => (
            if team.nickname.is_empty() {
                team.username.clone()
            } else {
                team.nickname.clone()
            },
            team.profile_url.clone(),
            team.description.clone(),
        ),
        None => (teamname.clone(), String::new(), String::new()),
    };

    let selected_category = use_context::<Signal<Option<String>>>();

    let list_btn_class = if view_mode() == HomeViewMode::List {
        "bg-[#1a1a1a]"
    } else {
        "bg-[#282828] hover:bg-[#222222]"
    };
    let card_btn_class = if view_mode() == HomeViewMode::Card {
        "bg-[#1a1a1a]"
    } else {
        "bg-[#282828] hover:bg-[#222222]"
    };

    rsx! {
        div { class: "flex flex-col w-full gap-6",
            TeamHeader {
                display_name,
                profile_url,
                description,
            }

            // View mode toggle + Create button
            div { class: "flex items-center justify-between w-full",
            div { class: "flex overflow-hidden rounded-[10px] w-fit",
                button {
                    class: "flex items-center justify-center w-[60px] h-[44px] cursor-pointer transition-colors {list_btn_class}",
                    onclick: move |_| view_mode.set(HomeViewMode::List),
                    icons::alignments::AlignJustify {
                        class: "w-6 h-6 [&>path]:stroke-icon-primary",
                    }
                }
                button {
                    class: "flex items-center justify-center w-[60px] h-[44px] cursor-pointer transition-colors {card_btn_class}",
                    onclick: move |_| view_mode.set(HomeViewMode::Card),
                    lucide_dioxus::LayoutGrid {
                        class: "w-6 h-6 [&>rect]:stroke-icon-primary [&>path]:stroke-icon-primary",
                    }
                }
            }

            // Create button
            button {
                class: "flex items-center gap-2.5 bg-white hover:bg-neutral-200 text-neutral-900 light:bg-[#404040] light:hover:bg-neutral-700 light:text-white px-5 py-3 h-[44px] rounded-full text-sm font-medium transition-colors cursor-pointer",
                onclick: move |_| {
                    let team_pk = team_pk_str.clone();
                    let nav = nav.clone();
                    async move {
                        let team_id = team_pk.map(|pk| pk.parse().unwrap_or_default());
                        match create_post_handler(team_id).await {
                            Ok(resp) => {
                                let post_pk: FeedPartition = resp.post_pk.into();
                                nav.push(format!("/posts/{post_pk}/edit"));
                            }
                            Err(e) => {
                                debug!("Failed to create post: {:?}", e);
                            }
                        }
                    }
                },
                icons::edit::Edit1 { class: "w-4 h-4 [&>path]:stroke-neutral-900 light:[&>path]:stroke-white" }
                span { "{tr.create}" }
            }
            } // end flex items-center justify-between

            // Posts
            TeamPostsPanel {
                key: "posts-{teamname}",
                teamname: teamname.clone(),
                view_mode: view_mode(),
                selected_category: selected_category(),
            }
        }
    }
}
