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

    // Latest analysis row drives the default lock state. While the
    // row is `InProgress` the form is hard-locked — backend is busy,
    // editing has no effect anyway. After `Finish` the form is
    // soft-locked: inputs are disabled and show the row's stored
    // params, BUT the 초기화 / 확인 buttons stay visible so the user
    // can re-edit and re-run. Clicking 초기화 flips
    // `is_editing` true and unlocks the form; clicking 확인 fires a
    // new analysis and re-locks once the row turns over. `Failed` and
    // "no row yet" stay editable from the start.
    let history = ctrl.discussion_results.read().clone();
    let latest = history.items.first().cloned();
    let mut is_editing = use_signal(|| false);
    let row_running = matches!(
        latest.as_ref().map(|r| r.status.clone()),
        Some(AnalyzeReportStatus::InProgress)
    );
    let row_finished = matches!(
        latest.as_ref().map(|r| r.status.clone()),
        Some(AnalyzeReportStatus::Finish)
    );
    let is_running = row_running || ctrl.handle_run_discussion.pending();
    // Effective lock: running always locks; finished locks only when
    // the user hasn't asked to edit yet.
    let locked = row_running || (row_finished && !*is_editing.read());

    // Source of truth for the displayed values:
    //   locked → the row's stored params (= what the backend ran with)
    //   else   → the live signal the user is editing
    let signal_params = ctrl.params.read().clone();
    let params: DiscussionAnalysisParams = if locked {
        latest
            .as_ref()
            .map(|r| r.params.clone())
            .unwrap_or_else(|| signal_params.clone())
    } else {
        signal_params.clone()
    };

    // Raw text buffer for the excluded-keywords input. Mirrors the
    // displayed params so locking instantly reflects the row's value.
    // Without this, every keystroke would split→join and the comma
    // the user just pressed would vanish before they could type the
    // next word.
    let mut excluded_text = use_signal(|| params.excluded_keywords.join(", "));
    if locked {
        let want = params.excluded_keywords.join(", ");
        if *excluded_text.read() != want {
            excluded_text.set(want);
        }
    }

    // Reset counter — bumped on 초기화. Drives `key` on each input so
    // the DOM nodes are torn down and rebuilt with the freshly-set
    // signal values; without this, controlled inputs sometimes hold
    // onto the typed text even after the underlying signal cleared.
    let mut reset_key = use_signal(|| 0u32);

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
                        key: "lda-{reset_key}",
                        class: "field__input",
                        r#type: "number",
                        min: "1",
                        max: "20",
                        value: "{params.num_topics}",
                        disabled: locked,
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
                        key: "tfidf-{reset_key}",
                        class: "field__input",
                        r#type: "number",
                        min: "1",
                        max: "20",
                        value: "{params.top_n_tfidf}",
                        disabled: locked,
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
                        key: "network-{reset_key}",
                        class: "field__input",
                        r#type: "number",
                        min: "1",
                        max: "30",
                        value: "{params.top_n_network}",
                        disabled: locked,
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
                        key: "excluded-{reset_key}",
                        class: "field__input",
                        r#type: "text",
                        placeholder: "{tr.detail_discussion_excluded_placeholder}",
                        value: "{excluded_text}",
                        disabled: locked,
                        oninput: move |evt| {
                            // Just buffer the raw string — parsing
                            // happens on submit so the user can type
                            // commas freely.
                            excluded_text.set(evt.value());
                        },
                    }
                    span { class: "field__hint", "{tr.detail_discussion_excluded_hint}" }
                }
            }
            div { class: "settings-foot",
                if is_running {
                    span { class: "settings-running",
                        "{tr.detail_discussion_running_title}"
                    }
                } else {
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
                            excluded_text.set(String::new());
                            // Unlock the form so the user can actually
                            // edit the just-reset values when coming
                            // off a `Finish` state.
                            is_editing.set(true);
                            // Force the four inputs to remount so their
                            // displayed `value` snaps back to the freshly
                            // reset signal — without the key bump
                            // controlled `<input>` elements sometimes
                            // hold onto whatever the user last typed.
                            reset_key.with_mut(|k| *k = k.wrapping_add(1));
                        },
                        "{tr.detail_discussion_btn_reset}"
                    }
                    button {
                        class: "btn btn--primary",
                        r#type: "button",
                        onclick: move |_| {
                            // Parse the raw text into canonical Vec<String>
                            // exactly once, on submit.
                            let parts: Vec<String> = excluded_text
                                .read()
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .filter(|s| !s.is_empty())
                                .collect();
                            ctrl.params.with_mut(|p| p.excluded_keywords = parts);
                            ctrl.handle_run_discussion.call();
                            // Re-lock once the run is queued — the new
                            // row will drive `locked` again as soon as
                            // the loader refreshes.
                            is_editing.set(false);
                        },
                        "{tr.detail_discussion_btn_apply}"
                    }
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
            LdaCard { row: row.clone() }
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
fn LdaCard(row: SpaceAnalyzeDiscussionResult) -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();
    let mut ctrl = use_context::<UseAnalyzeReportDetail>();
    let mut toast = use_toast();

    // Edit toggle + scratch buffer of new names. Keyed by the
    // *current* topic label (whatever's in the row right now —
    // could be the auto-generated `토픽_N` or a previously saved
    // custom name). Cleared on save so the next edit cycle starts
    // from the freshly-stored state.
    let mut editing = use_signal(|| false);
    let mut renames = use_signal::<std::collections::HashMap<String, String>>(
        std::collections::HashMap::new,
    );

    let count = row.topics.len();
    let space_id = ctrl.space_id;
    let report_id = ctrl.report_id;
    let row_for_save = row.clone();

    let mut save_action = use_action(move |new_topics: Vec<TopicRow>| {
        let row = row_for_save.clone();
        async move {
            let report_typed: SpaceAnalyzeReportEntityType = report_id().into();
            let discussion_typed: FeedPartition = row.discussion_id.clone().into();
            update_discussion_topics(
                space_id(),
                report_typed,
                discussion_typed,
                row.request_id.clone(),
                UpdateDiscussionTopicsRequest { topics: new_topics },
            )
            .await?;
            ctrl.discussion_results.restart();
            Ok::<(), crate::common::Error>(())
        }
    });

    let topics_for_save = row.topics.clone();
    let renames_signal_for_save = renames;

    rsx! {
        section { class: "card",
            div { class: "card__head",
                div { class: "card__title",
                    "{tr.detail_lda_card_title} ({count}{tr.detail_lda_card_count_suffix})"
                }
                button {
                    class: "card__action",
                    r#type: "button",
                    "aria-pressed": "{editing}",
                    onclick: move |_| {
                        let next = !editing();
                        editing.set(next);
                        if !next {
                            renames.write().clear();
                        }
                    },
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        path { d: "M12 20h9" }
                        path { d: "M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4 12.5-12.5z" }
                    }
                }
            }
            if row.topics.is_empty() {
                div { class: "card__hint", "{tr.detail_panel_empty_text_answers}" }
            } else {
                div {
                    class: "topic-table",
                    id: "lda-table",
                    "data-edit": "{editing}",
                    div { class: "topic-table__head",
                        span { "{tr.detail_lda_col_topic}" }
                        span { "{tr.detail_lda_col_keywords}" }
                    }
                    for (idx, topic) in row.topics.iter().enumerate() {
                        {
                            let kws = topic.keywords.join(", ");
                            let original_id = topic.topic.clone();
                            let display_name = renames
                                .read()
                                .get(&original_id)
                                .cloned()
                                .unwrap_or_else(|| original_id.clone());
                            let id_for_input = original_id.clone();
                            rsx! {
                                div { key: "lda-{idx}", class: "topic-table__row",
                                    span { class: "topic-table__id", "{display_name}" }
                                    input {
                                        class: "topic-table__id-input",
                                        r#type: "text",
                                        value: "{display_name}",
                                        oninput: move |evt| {
                                            let val = evt.value();
                                            let key = id_for_input.clone();
                                            renames
                                                .with_mut(|map| {
                                                    if val.trim().is_empty() {
                                                        map.remove(&key);
                                                    } else {
                                                        map.insert(key, val);
                                                    }
                                                });
                                        },
                                    }
                                    span { class: "topic-table__keywords", "{kws}" }
                                }
                            }
                        }
                    }
                }
                // 저장 버튼 — 편집 모드일 때만 노출. 클릭 시 변경된 라벨을
                // DB 에 PATCH 하고 카드를 결과 새로고침까지 끝낸 뒤 편집 모드 종료.
                if editing() {
                    div { class: "topic-table-foot",
                        button {
                            class: "btn btn--primary",
                            r#type: "button",
                            disabled: save_action.pending(),
                            onclick: move |_| {
                                let map = renames_signal_for_save.read().clone();
                                let merged: Vec<TopicRow> = topics_for_save
                                    .iter()
                                    .map(|t| {
                                        TopicRow {
                                            topic: map
                                                .get(&t.topic)
                                                .cloned()
                                                .unwrap_or_else(|| t.topic.clone()),
                                            keywords: t.keywords.clone(),
                                        }
                                    })
                                    .collect();
                                save_action.call(merged);
                                editing.set(false);
                                renames.write().clear();
                                toast.info("토픽 이름을 저장했습니다");
                            },
                            "저장"
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
                div { class: "card__title",
                    "{tr.detail_network_card_title} ({node_count}{tr.detail_network_card_count_suffix})"
                }
                span { class: "card__count",
                    "{node_count}{tr.detail_network_card_count_suffix} · {edge_count}{tr.detail_network_card_edge_suffix}"
                }
            }
            if nodes.is_empty() {
                div { class: "card__hint", "{tr.detail_panel_empty_text_answers}" }
            } else {
                div { class: "network-wrap",
                    NetworkChart { nodes: nodes.clone(), edges: edges.clone() }
                }
            }
        }
    }
}

/// Force-directed layout SVG of the word co-occurrence graph
/// (Fruchterman-Reingold). Heavy nodes find natural cluster centres,
/// loose nodes drift to the periphery — same look as the second
/// reference image the user shared (organic clusters, varied node
/// sizing). Pure Rust, no JS engine.
///
/// Bubble radius scales with per-document frequency. Edge stroke
/// width scales with co-occurrence weight. Initial layout is
/// deterministic (no RNG) so successive renders of the same data
/// always produce the same picture.
#[component]
fn NetworkChart(nodes: Vec<NetworkNode>, edges: Vec<NetworkEdge>) -> Element {
    let n = nodes.len();
    if n == 0 {
        return rsx! {};
    }

    let width: f64 = 700.0;
    let height: f64 = 580.0;
    let max_node_w = nodes.iter().map(|n| n.weight).max().unwrap_or(1).max(1) as f64;
    let max_edge_w = edges.iter().map(|e| e.weight).max().unwrap_or(1).max(1) as f64;

    // term → index lookup so edges can resolve to position slots.
    let term_idx: std::collections::HashMap<String, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, node)| (node.term.clone(), i))
        .collect();
    let edge_pairs: Vec<(usize, usize, f64)> = edges
        .iter()
        .filter_map(|e| {
            let i = *term_idx.get(&e.source)?;
            let j = *term_idx.get(&e.target)?;
            Some((i, j, e.weight as f64))
        })
        .collect();

    // ── Fruchterman-Reingold ─────────────────────────────────
    let mut pos: Vec<(f64, f64)> = (0..n)
        .map(|i| {
            // Deterministic spread + light per-index perturbation so
            // simulation has somewhere to go (a perfect ring start
            // settles too quickly into a near-perfect ring).
            let theta =
                (i as f64 / n as f64) * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2;
            let r = (width.min(height) / 2.0) * 0.45;
            let cx = width / 2.0;
            let cy = height / 2.0;
            let perturb_x = (((i * 73 + 17) % 41) as f64) - 20.0;
            let perturb_y = (((i * 113 + 31) % 41) as f64) - 20.0;
            (
                cx + r * theta.cos() + perturb_x,
                cy + r * theta.sin() + perturb_y,
            )
        })
        .collect();

    let area = width * height;
    let k = (area / n as f64).sqrt(); // ideal spring length
    let mut t = width / 8.0; // temperature — limits per-iter movement
    let cooling = 0.94;
    let iterations = 120;

    for _ in 0..iterations {
        let mut disp = vec![(0.0_f64, 0.0_f64); n];

        // Repulsive force between every pair: f = k^2 / d.
        for v in 0..n {
            for u in 0..n {
                if u == v {
                    continue;
                }
                let dx = pos[v].0 - pos[u].0;
                let dy = pos[v].1 - pos[u].1;
                let d = (dx * dx + dy * dy).sqrt().max(0.01);
                let f = (k * k) / d;
                disp[v].0 += (dx / d) * f;
                disp[v].1 += (dy / d) * f;
            }
        }

        // Attractive force along edges, weighted by co-occurrence:
        // heavier edges pull harder so co-frequent terms cluster.
        for (i, j, w) in &edge_pairs {
            let dx = pos[*j].0 - pos[*i].0;
            let dy = pos[*j].1 - pos[*i].1;
            let d = (dx * dx + dy * dy).sqrt().max(0.01);
            let weight_norm = w / max_edge_w;
            let f = (d * d / k) * weight_norm;
            disp[*i].0 += (dx / d) * f;
            disp[*i].1 += (dy / d) * f;
            disp[*j].0 -= (dx / d) * f;
            disp[*j].1 -= (dy / d) * f;
        }

        // Apply, capped at temperature, clamped to viewport.
        for v in 0..n {
            let dx = disp[v].0;
            let dy = disp[v].1;
            let d = (dx * dx + dy * dy).sqrt();
            if d > 0.0 {
                let limit = d.min(t);
                pos[v].0 += (dx / d) * limit;
                pos[v].1 += (dy / d) * limit;
            }
            pos[v].0 = pos[v].0.clamp(50.0, width - 50.0);
            pos[v].1 = pos[v].1.clamp(50.0, height - 50.0);
        }

        t *= cooling;
    }

    // ── Render ────────────────────────────────────────────────
    rsx! {
        svg {
            class: "network-svg",
            view_box: "0 0 700 580",
            preserve_aspect_ratio: "xMidYMid meet",
            "aria-label": "Word co-occurrence network",
            // Edges first so bubbles paint on top.
            g { class: "network-links",
                for (idx, edge) in edges.iter().enumerate() {
                    {
                        let a = term_idx.get(&edge.source).map(|i| pos[*i]);
                        let b = term_idx.get(&edge.target).map(|i| pos[*i]);
                        match (a, b) {
                            (Some((x1, y1)), Some((x2, y2))) => {
                                let sw = 0.6 + (edge.weight as f64 / max_edge_w) * 2.4;
                                rsx! {
                                    line {
                                        key: "edge-{idx}",
                                        class: "network-link",
                                        x1: "{x1:.1}",
                                        y1: "{y1:.1}",
                                        x2: "{x2:.1}",
                                        y2: "{y2:.1}",
                                        stroke_width: "{sw:.2}",
                                    }
                                }
                            }
                            _ => rsx! {},
                        }
                    }
                }
            }
            g { class: "network-nodes",
                for (idx, node) in nodes.iter().enumerate() {
                    {
                        let (x, y) = pos[idx];
                        // Bubble radius 14..38 px by per-document frequency.
                        let r = 14.0 + (node.weight as f64 / max_node_w) * 24.0;
                        rsx! {
                            g { key: "node-{idx}",
                                circle {
                                    class: "network-bubble",
                                    cx: "{x:.1}",
                                    cy: "{y:.1}",
                                    r: "{r:.1}",
                                }
                                text {
                                    class: "network-bubble-text",
                                    x: "{x:.1}",
                                    y: "{y:.1}",
                                    "{node.term}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
