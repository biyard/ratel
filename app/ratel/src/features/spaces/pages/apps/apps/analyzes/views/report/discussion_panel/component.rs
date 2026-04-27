use crate::features::spaces::pages::apps::apps::analyzes::*;

/// Discussion panel — settings card, TF-IDF chart, LDA topic table,
/// text network. Mockup data 1:1 with `analyze-detail-arena.html`.
#[component]
pub fn DiscussionPanel() -> Element {
    let tr: SpaceAnalyzesAppTranslate = use_translate();

    let tfidf_rows: Vec<(&'static str, &'static str, &'static str)> = vec![
        ("증거", "100%", "1.79"),
        ("진술", "95.5%", "1.71"),
        ("피해자", "83.8%", "1.50"),
        ("처벌", "55.9%", "1.00"),
        ("필요", "55.3%", "0.99"),
        ("의무", "51.4%", "0.92"),
        ("강화", "49.2%", "0.88"),
        ("기소", "49.2%", "0.88"),
        ("무고죄", "45.8%", "0.82"),
        ("가능", "45.8%", "0.82"),
        ("무고", "45.3%", "0.81"),
        ("진술 기소", "45.3%", "0.81"),
        ("녹음", "42.5%", "0.76"),
        ("간음죄", "40.2%", "0.72"),
        ("비동", "40.2%", "0.72"),
        ("비동 간음죄", "40.2%", "0.72"),
        ("수사", "38.5%", "0.69"),
        ("신고", "38.0%", "0.68"),
        ("녹음 녹화", "36.9%", "0.66"),
        ("녹화", "36.9%", "0.66"),
    ];

    let lda_rows: Vec<(&'static str, &'static str)> = vec![
        ("토픽_1", "선고, 방향, 문제, 입증, 공정, 과정, 입장, 가해자, 해당, 동의"),
        ("토픽_2", "침해, 논의, 방향, 문제, 입증, 선고, 공정, 과정, 입장, 가해자"),
        ("토픽_3", "장치, 논의, 신중, 방향, 문제, 입증, 선고, 공정, 과정, 입장"),
        ("토픽_4", "균형, 부족, 입장, 방향, 문제, 입증, 선고, 공정, 과정, 가해자"),
        ("토픽_5", "입장, 가해자, 험이, 방향, 문제, 입증, 선고, 공정, 과정, 해당"),
        ("토픽_6", "동의, 방향, 문제, 입증, 선고, 공정, 과정, 입장, 가해자, 해당"),
        ("토픽_7", "형사, 반복, 인정, 지원, 방향, 문제, 입증, 선고, 공정, 과정"),
        ("토픽_8", "방향, 문제, 해당, 과정, 위축, 입증, 선고, 공정, 입장, 가해자"),
        ("토픽_9", "조사, 가이드라인, 도입, 기록, 방향, 문제, 입증, 선고, 공정, 과정"),
        ("토픽_10", "입증, 공정, 원칙, 위험, 방향, 문제, 선고, 과정, 입장, 가해자"),
    ];

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
                span { "data-discussion-title": true, "{tr.detail_discussion_title}" }
            }

            // Settings card
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
                            value: "10",
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
                            value: "20",
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
                            value: "15",
                        }
                        span { class: "field__hint", "{tr.detail_discussion_network_hint}" }
                    }
                    div { class: "field",
                        label { class: "field__label", "{tr.detail_discussion_excluded_label}" }
                        input {
                            class: "field__input",
                            r#type: "text",
                            placeholder: "{tr.detail_discussion_excluded_placeholder}",
                        }
                        span { class: "field__hint", "{tr.detail_discussion_excluded_hint}" }
                    }
                }
                div { class: "settings-foot",
                    button { class: "btn btn--ghost", "{tr.detail_discussion_btn_reset}" }
                    button { class: "btn btn--primary", "{tr.detail_discussion_btn_apply}" }
                }
            }

            // TF-IDF card
            section { class: "card",
                div { class: "card__head",
                    div { class: "card__title", "{tr.detail_tfidf_card_title}" }
                    span { class: "card__count", "{tr.detail_tfidf_card_count}" }
                }
                div { class: "tfidf-chart",
                    for (idx, (kw, w, v)) in tfidf_rows.iter().enumerate() {
                        button {
                            key: "tfidf-{idx}",
                            r#type: "button",
                            class: "tfidf-row",
                            "aria-pressed": "false",
                            "data-filter-source": "discussion",
                            "data-filter-kind": "keyword",
                            "data-filter-value": "{kw}",
                            span { class: "tfidf-row__keyword", "{kw}" }
                            div { class: "tfidf-row__track",
                                div {
                                    class: "tfidf-row__fill",
                                    style: "width: {w};",
                                }
                            }
                            span { class: "tfidf-row__value", "{v}" }
                        }
                    }
                }
            }

            // LDA topic table
            section { class: "card",
                div { class: "card__head",
                    div { class: "card__title", "{tr.detail_lda_card_title}" }
                    button {
                        class: "card__action",
                        r#type: "button",
                        "aria-label": "{tr.detail_lda_edit_label}",
                        "data-edit-target": "lda-table",
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
                div {
                    class: "topic-table",
                    id: "lda-table",
                    "data-edit": "false",
                    div { class: "topic-table__head",
                        span { "{tr.detail_lda_col_topic}" }
                        span { "{tr.detail_lda_col_keywords}" }
                        span { "aria-hidden": "true", "{tr.detail_lda_col_filter}" }
                    }
                    for (idx, (id, kws)) in lda_rows.iter().enumerate() {
                        div { key: "lda-{idx}", class: "topic-table__row",
                            span { class: "topic-table__id", "{id}" }
                            input {
                                class: "topic-table__id-input",
                                r#type: "text",
                                value: "{id}",
                            }
                            span { class: "topic-table__keywords", "{kws}" }
                            button {
                                class: "topic-table__filter",
                                r#type: "button",
                                "aria-pressed": "false",
                                "aria-label": "{tr.detail_lda_filter_aria_prefix} {id}",
                                "data-filter-source": "discussion",
                                "data-filter-kind": "topic",
                                "data-filter-value": "{id}",
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

            // Text network
            section { class: "card",
                div { class: "card__head",
                    div { class: "card__title", "{tr.detail_network_card_title}" }
                    span { class: "card__count", "{tr.detail_network_card_count}" }
                }
                div { class: "network-wrap",
                    NetworkSvg { aria_label: tr.detail_network_aria.to_string() }
                }
            }
        }
    }
}

/// Static SVG bubbles + links — copied verbatim from the mockup.
#[component]
fn NetworkSvg(aria_label: String) -> Element {
    rsx! {
        svg {
            class: "network-svg",
            view_box: "0 0 700 580",
            preserve_aspect_ratio: "xMidYMid meet",
            "aria-label": "{aria_label}",
            g { class: "network-links",
                line {
                    class: "network-link",
                    x1: "330",
                    y1: "260",
                    x2: "520",
                    y2: "260",
                }
                line {
                    class: "network-link",
                    x1: "330",
                    y1: "260",
                    x2: "140",
                    y2: "260",
                }
                line {
                    class: "network-link",
                    x1: "330",
                    y1: "260",
                    x2: "330",
                    y2: "440",
                }
                line {
                    class: "network-link",
                    x1: "330",
                    y1: "260",
                    x2: "440",
                    y2: "420",
                }
                line {
                    class: "network-link",
                    x1: "330",
                    y1: "260",
                    x2: "240",
                    y2: "110",
                }
                line {
                    class: "network-link",
                    x1: "330",
                    y1: "260",
                    x2: "420",
                    y2: "140",
                }
                line {
                    class: "network-link",
                    x1: "330",
                    y1: "260",
                    x2: "180",
                    y2: "400",
                }
                line {
                    class: "network-link",
                    x1: "520",
                    y1: "260",
                    x2: "600",
                    y2: "420",
                }
                line {
                    class: "network-link",
                    x1: "520",
                    y1: "260",
                    x2: "420",
                    y2: "140",
                }
                line {
                    class: "network-link",
                    x1: "520",
                    y1: "260",
                    x2: "440",
                    y2: "420",
                }
                line {
                    class: "network-link",
                    x1: "140",
                    y1: "260",
                    x2: "80",
                    y2: "400",
                }
                line {
                    class: "network-link",
                    x1: "140",
                    y1: "260",
                    x2: "240",
                    y2: "110",
                }
                line {
                    class: "network-link",
                    x1: "140",
                    y1: "260",
                    x2: "180",
                    y2: "400",
                }
                line {
                    class: "network-link",
                    x1: "330",
                    y1: "440",
                    x2: "440",
                    y2: "420",
                }
                line {
                    class: "network-link",
                    x1: "330",
                    y1: "440",
                    x2: "180",
                    y2: "400",
                }
                line {
                    class: "network-link",
                    x1: "330",
                    y1: "440",
                    x2: "270",
                    y2: "530",
                }
                line {
                    class: "network-link",
                    x1: "420",
                    y1: "140",
                    x2: "620",
                    y2: "160",
                }
                line {
                    class: "network-link",
                    x1: "420",
                    y1: "140",
                    x2: "520",
                    y2: "100",
                }
                line {
                    class: "network-link",
                    x1: "240",
                    y1: "110",
                    x2: "80",
                    y2: "130",
                }
                line {
                    class: "network-link",
                    x1: "240",
                    y1: "110",
                    x2: "370",
                    y2: "70",
                }
                line {
                    class: "network-link",
                    x1: "440",
                    y1: "420",
                    x2: "600",
                    y2: "420",
                }
                line {
                    class: "network-link",
                    x1: "80",
                    y1: "400",
                    x2: "180",
                    y2: "400",
                }
            }
            g { class: "network-bubbles",
                circle {
                    class: "network-bubble",
                    cx: "520",
                    cy: "260",
                    r: "62",
                }
                text {
                    class: "network-bubble-text",
                    x: "520",
                    y: "260",
                    font_size: "18",
                    "신고"
                }
                circle {
                    class: "network-bubble",
                    cx: "140",
                    cy: "260",
                    r: "56",
                }
                text {
                    class: "network-bubble-text",
                    x: "140",
                    y: "260",
                    font_size: "16",
                    "무고죄"
                }
                circle {
                    class: "network-bubble",
                    cx: "330",
                    cy: "440",
                    r: "52",
                }
                text {
                    class: "network-bubble-text",
                    x: "330",
                    y: "440",
                    font_size: "16",
                    "의무"
                }
                circle {
                    class: "network-bubble",
                    cx: "330",
                    cy: "260",
                    r: "46",
                }
                text {
                    class: "network-bubble-text",
                    x: "330",
                    y: "260",
                    font_size: "14",
                    "수사"
                }
                circle {
                    class: "network-bubble",
                    cx: "420",
                    cy: "140",
                    r: "40",
                }
                text {
                    class: "network-bubble-text",
                    x: "420",
                    y: "140",
                    font_size: "13",
                    "증거"
                }
                circle {
                    class: "network-bubble",
                    cx: "440",
                    cy: "420",
                    r: "38",
                }
                text {
                    class: "network-bubble-text",
                    x: "440",
                    y: "420",
                    font_size: "13",
                    "처벌"
                }
                circle {
                    class: "network-bubble",
                    cx: "240",
                    cy: "110",
                    r: "34",
                }
                text {
                    class: "network-bubble-text",
                    x: "240",
                    y: "110",
                    font_size: "12",
                    "진술"
                }
                circle {
                    class: "network-bubble",
                    cx: "370",
                    cy: "70",
                    r: "30",
                }
                text {
                    class: "network-bubble-text",
                    x: "370",
                    y: "70",
                    font_size: "12",
                    "녹음"
                }
                circle {
                    class: "network-bubble",
                    cx: "80",
                    cy: "400",
                    r: "28",
                }
                text {
                    class: "network-bubble-text",
                    x: "80",
                    y: "400",
                    font_size: "11",
                    "녹화"
                }
                circle {
                    class: "network-bubble",
                    cx: "180",
                    cy: "400",
                    r: "26",
                }
                text {
                    class: "network-bubble-text",
                    x: "180",
                    y: "400",
                    font_size: "11",
                    "강화"
                }
                circle {
                    class: "network-bubble",
                    cx: "600",
                    cy: "420",
                    r: "26",
                }
                text {
                    class: "network-bubble-text",
                    x: "600",
                    y: "420",
                    font_size: "11",
                    "피해자"
                }
                circle {
                    class: "network-bubble",
                    cx: "620",
                    cy: "160",
                    r: "24",
                }
                text {
                    class: "network-bubble-text",
                    x: "620",
                    y: "160",
                    font_size: "11",
                    "기소"
                }
                circle {
                    class: "network-bubble",
                    cx: "80",
                    cy: "130",
                    r: "22",
                }
                text {
                    class: "network-bubble-text",
                    x: "80",
                    y: "130",
                    font_size: "10",
                    "무고"
                }
                circle {
                    class: "network-bubble",
                    cx: "520",
                    cy: "100",
                    r: "20",
                }
                text {
                    class: "network-bubble-text",
                    x: "520",
                    y: "100",
                    font_size: "10",
                    "필요"
                }
                circle {
                    class: "network-bubble",
                    cx: "270",
                    cy: "530",
                    r: "22",
                }
                text {
                    class: "network-bubble-text",
                    x: "270",
                    y: "530",
                    font_size: "10",
                    "성범죄"
                }
            }
        }
    }
}
