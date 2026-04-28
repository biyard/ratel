use crate::features::spaces::pages::apps::apps::analyzes::*;

#[component]
pub fn DiscussionPanel() -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_context::<UseAnalyzeReportDetail>();

    let selected = ctrl.selected_discussion.read().clone();
    let history = ctrl.discussion_results.read().clone();
    let latest = history.items.first().cloned();

    let panel_title = ctrl
        .detail
        .read()
        .clone()
        .discussions
        .iter()
        .find(|d| Some(d.discussion_id.to_string()) == selected)
        .map(|d| d.title.clone())
        .unwrap_or_else(|| tr.detail_discussion_title.to_string());

    rsx! {
        section {
            class: "panel",
            "data-panel": "discussion",
            "data-active": "false",
            h1 { class: "main-title",
                span { class: "main-title__chip main-title__chip--discussion",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                    }
                    "{tr.detail_panel_chip_discussion}"
                }
                span { "data-discussion-title": true, "{panel_title}" }
            }

            // Empty state — user hasn't picked a discussion from the
            // sidebar yet. Hide the entire form/result stack since
            // none of them are meaningful without context.
            if selected.is_none() {
                section { class: "card",
                    div { class: "card__head",
                        div { class: "card__title", "{tr.detail_discussion_pick_title}" }
                    }
                    div { class: "card__hint", "{tr.detail_discussion_pick_hint}" }
                }
            } else {
                DiscussionSettingsCard {}
                DiscussionResultsView { latest: latest.clone() }
            }
        }
    }
}

#[component]
fn DiscussionSettingsCard() -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_context::<UseAnalyzeReportDetail>();

    let params = ctrl.params.read().clone();
    let excluded_text = params.excluded_keywords.join(", ");

    rsx! {
        section { class: "card",
            div { class: "card__head",
                div { class: "card__title", "{tr.detail_discussion_settings_title}" }
                span { class: "card__count", "{tr.detail_discussion_topic_modeling_label}" }
            }
            div { class: "settings-grid",
                div { class: "field",
                    label { class: "field__label", "{tr.detail_discussion_lda_label}" }
                    input {
                        class: "field__input",
                        r#type: "number",
                        min: "1",
                        max: "20",
                        value: "{params.num_topics}",
                        oninput: move |evt| {
                            if let Ok(n) = evt.value().parse::<usize>() {
                                ctrl.params.with_mut(|p| p.num_topics = n.clamp(1, 20));
                            }
                        },
                    }
                    span { class: "field__hint", "{tr.detail_discussion_lda_hint}" }
                }
                div { class: "field",
                    label { class: "field__label", "{tr.detail_discussion_tfidf_label}" }
                    input {
                        class: "field__input",
                        r#type: "number",
                        min: "1",
                        max: "20",
                        value: "{params.top_n_tfidf}",
                        oninput: move |evt| {
                            if let Ok(n) = evt.value().parse::<usize>() {
                                ctrl.params.with_mut(|p| p.top_n_tfidf = n.clamp(1, 20));
                            }
                        },
                    }
                    span { class: "field__hint", "{tr.detail_discussion_lda_hint}" }
                }
                div { class: "field",
                    label { class: "field__label", "{tr.detail_discussion_network_label}" }
                    input {
                        class: "field__input",
                        r#type: "number",
                        min: "1",
                        max: "30",
                        value: "{params.top_n_network}",
                        oninput: move |evt| {
                            if let Ok(n) = evt.value().parse::<usize>() {
                                ctrl.params.with_mut(|p| p.top_n_network = n.clamp(1, 30));
                            }
                        },
                    }
                    span { class: "field__hint", "{tr.detail_discussion_network_hint}" }
                }
                div { class: "field",
                    label { class: "field__label", "{tr.detail_discussion_excluded_label}" }
                    input {
                        class: "field__input",
                        r#type: "text",
                        placeholder: "{tr.detail_discussion_excluded_placeholder}",
                        value: "{excluded_text}",
                        oninput: move |evt| {
                            let raw = evt.value();
                            let parts: Vec<String> = raw
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                            ctrl.params.with_mut(|p| p.excluded_keywords = parts);
                        },
                    }
                    span { class: "field__hint", "{tr.detail_discussion_excluded_hint}" }
                }
            }
            div { class: "settings-foot",
                button {
                    class: "btn btn--ghost",
                    r#type: "button",
                    onclick: move |_| {
                        ctrl.params
                            .set(DiscussionAnalysisParams {
                                num_topics: 10,
                                top_n_tfidf: 20,
                                top_n_network: 15,
                                excluded_keywords: Vec::new(),
                            });
                    },
                    "{tr.detail_discussion_btn_reset}"
                }
                button {
                    class: "btn btn--primary",
                    r#type: "button",
                    onclick: move |_| ctrl.handle_run_discussion.call(),
                    "{tr.detail_discussion_btn_apply}"
                }
            }
        }
    }
}

#[component]
fn DiscussionResultsView(latest: Option<SpaceAnalyzeDiscussionResult>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();

    let row = match latest {
        Some(r) => r,
        None => {
            return rsx! {
                section { class: "card",
                    div { class: "card__head",
                        div { class: "card__title", "{tr.detail_discussion_no_run_title}" }
                    }
                    div { class: "card__hint", "{tr.detail_discussion_no_run_hint}" }
                }
            };
        }
    };

    match row.status {
        AnalyzeReportStatus::InProgress => rsx! {
            section { class: "card",
                div { class: "card__head",
                    div { class: "card__title", "{tr.detail_discussion_running_title}" }
                }
                div { class: "card__hint", "{tr.detail_discussion_running_hint}" }
            }
        },
        AnalyzeReportStatus::Failed => rsx! {
            section { class: "card",
                div { class: "card__head",
                    div { class: "card__title", "{tr.detail_discussion_failed_title}" }
                }
                div { class: "card__hint", "{tr.detail_discussion_failed_hint}" }
            }
        },
        AnalyzeReportStatus::Finish => rsx! {
            TfidfCard { rows: row.tfidf_terms.clone() }
            LdaCard { rows: row.topics.clone() }
            NetworkCard {
                nodes: row.network_nodes.clone(),
                edges: row.network_edges.clone(),
            }
        },
    }
}

#[component]
fn TfidfCard(rows: Vec<TermScore>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let count = rows.len();
    rsx! {
        section { class: "card",
            div { class: "card__head",
                div { class: "card__title", "{tr.detail_tfidf_card_title}" }
                span { class: "card__count", "{count}{tr.detail_tfidf_card_count_suffix}" }
            }
            if rows.is_empty() {
                div { class: "card__hint", "{tr.detail_panel_empty_text_answers}" }
            } else {
                div { class: "tfidf-chart",
                    for (idx, term) in rows.iter().enumerate() {
                        button {
                            key: "tfidf-{idx}",
                            r#type: "button",
                            class: "tfidf-row",
                            "aria-pressed": "false",
                            "data-filter-source": "discussion",
                            "data-filter-kind": "keyword",
                            "data-filter-value": "{term.term}",
                            span { class: "tfidf-row__keyword", "{term.term}" }
                            div { class: "tfidf-row__track",
                                div {
                                    class: "tfidf-row__fill",
                                    style: "width: {term.relative * 100.0:.1}%;",
                                }
                            }
                            span { class: "tfidf-row__value", "{term.score:.2}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LdaCard(rows: Vec<TopicRow>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    rsx! {
        section { class: "card",
            div { class: "card__head",
                div { class: "card__title", "{tr.detail_lda_card_title}" }
            }
            if rows.is_empty() {
                div { class: "card__hint", "{tr.detail_panel_empty_text_answers}" }
            } else {
                div {
                    class: "topic-table",
                    id: "lda-table",
                    "data-edit": "false",
                    div { class: "topic-table__head",
                        span { "{tr.detail_lda_col_topic}" }
                        span { "{tr.detail_lda_col_keywords}" }
                        span { "aria-hidden": "true", "{tr.detail_lda_col_filter}" }
                    }
                    for (idx, topic) in rows.iter().enumerate() {
                        {
                            let kws = topic.keywords.join(", ");
                            rsx! {
                                div { key: "lda-{idx}", class: "topic-table__row",
                                    span { class: "topic-table__id", "{topic.topic}" }
                                    input {
                                        class: "topic-table__id-input",
                                        r#type: "text",
                                        value: "{topic.topic}",
                                    }
                                    span { class: "topic-table__keywords", "{kws}" }
                                    button {
                                        class: "topic-table__filter",
                                        r#type: "button",
                                        "aria-pressed": "false",
                                        "data-filter-source": "discussion",
                                        "data-filter-kind": "topic",
                                        "data-filter-value": "{topic.topic}",
                                        svg {
                                            view_box: "0 0 24 24",
                                            fill: "none",
                                            stroke: "currentColor",
                                            "stroke-width": "2",
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            polygon { points: "22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NetworkCard(nodes: Vec<NetworkNode>, edges: Vec<NetworkEdge>) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let node_count = nodes.len();
    let edge_count = edges.len();
    rsx! {
        section { class: "card",
            div { class: "card__head",
                div { class: "card__title", "{tr.detail_network_card_title}" }
                span { class: "card__count", "{node_count} 노드 · {edge_count} 엣지" }
            }
            if nodes.is_empty() {
                div { class: "card__hint", "{tr.detail_panel_empty_text_answers}" }
            } else {
                // Simplified rendering: top nodes as badges with weight,
                // top edges as a list. The rich SVG visualisation in the
                // mock relied on hand-positioned coordinates — surfacing
                // the real numbers here is a fair stand-in until a
                // force-directed renderer lands.
                div { class: "network-summary",
                    div { class: "network-nodes",
                        for (idx, node) in nodes.iter().enumerate() {
                            span {
                                key: "node-{idx}",
                                class: "network-node-chip",
                                title: "{node.weight}",
                                "{node.term}"
                                span { class: "network-node-chip__weight", " · {node.weight}" }
                            }
                        }
                    }
                    if !edges.is_empty() {
                        ul { class: "network-edges",
                            for (idx, edge) in edges.iter().take(20).enumerate() {
                                li { key: "edge-{idx}", class: "network-edge",
                                    "{edge.source} ↔ {edge.target}"
                                    span { class: "network-edge__weight", " ({edge.weight})" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
