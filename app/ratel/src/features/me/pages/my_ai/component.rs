use super::*;
use crate::common::hooks::use_origin;
use crate::common::utils::time::sleep;
use crate::features::social::pages::user_setting::controllers::{
    get_mcp_secret_handler, regenerate_mcp_secret_handler,
};
use crate::*;
#[cfg(not(feature = "server"))]
use crate::common::{wasm_bindgen_futures, web_sys};

const MCP_DOCS_URL: &str = "https://modelcontextprotocol.io/";
const PLACEHOLDER_TOKEN: &str = "<your-token>";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GuideTab {
    ClaudeCode,
    ClaudeDesktop,
    Cursor,
    Generic,
}

#[component]
pub fn MyAiPage() -> Element {
    let tr: MyAiTranslate = use_translate();
    let nav = use_navigator();
    let origin = use_origin();

    let mut mcp_status =
        use_loader(move || async move { get_mcp_secret_handler().await })?;
    let has_secret = mcp_status().has_secret;

    // Raw token only available right after generate/regenerate.
    let mut raw_token = use_signal(|| Option::<String>::None);
    let mut generating = use_signal(|| false);
    // `url_copied` is mutated only inside the `web`-gated clipboard
    // branch of `on_copy_url`; mark `mut` so the signal can flip
    // `data-copied` after a successful clipboard write.
    #[allow(unused_mut)]
    let mut url_copied = use_signal(|| false);
    let mut error_message = use_signal(|| Option::<String>::None);
    let active_tab = use_signal(|| GuideTab::ClaudeCode);

    let mcp_url = raw_token().map(|token| format!("{}/mcp/{}", origin(), token));
    let display_url = mcp_url
        .clone()
        .unwrap_or_else(|| format!("{}/mcp/{}", origin(), PLACEHOLDER_TOKEN));

    let on_generate = move |_: MouseEvent| {
        spawn(async move {
            generating.set(true);
            error_message.set(None);
            match regenerate_mcp_secret_handler().await {
                Ok(resp) => {
                    raw_token.set(resp.secret);
                    mcp_status.restart();
                }
                Err(e) => {
                    error_message.set(Some(format!("{e}")));
                }
            }
            generating.set(false);
        });
    };

    let on_copy_url = {
        let mcp_url = mcp_url.clone();
        move |_: MouseEvent| {
            let Some(url) = mcp_url.clone() else {
                return;
            };
            #[cfg(feature = "web")]
            {
                let tr_err = tr.error_clipboard.to_string();
                spawn(async move {
                    if let Some(window) = web_sys::window() {
                        let clipboard = window.navigator().clipboard();
                        match wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&url))
                            .await
                        {
                            Ok(_) => {
                                url_copied.set(true);
                                sleep(std::time::Duration::from_millis(2000)).await;
                                url_copied.set(false);
                            }
                            Err(_) => {
                                error_message.set(Some(tr_err));
                            }
                        }
                    }
                });
            }
            #[cfg(not(feature = "web"))]
            {
                let _ = url;
            }
        }
    };

    let on_back = move |_: MouseEvent| {
        nav.go_back();
    };

    rsx! {
        SeoMeta { title: "{tr.page_title}" }

        div { class: "my-ai",
            // ── Topbar ───────────────────────────────
            header { class: "my-ai__topbar",
                div { class: "my-ai__topbar-left",
                    button {
                        class: "my-ai__back",
                        "aria-label": "{tr.back}",
                        "data-testid": "my-ai-back",
                        onclick: on_back,
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            path { d: "M15 18l-6-6 6-6" }
                        }
                    }
                    div { class: "my-ai__title",
                        span { class: "my-ai__title-main", "{tr.page_title}" }
                        span {
                            class: "my-ai__status",
                            "data-state": if has_secret { "on" } else { "off" },
                            if has_secret {
                                "{tr.status_online}"
                            } else {
                                "{tr.status_offline}"
                            }
                        }
                    }
                }
                a {
                    class: "my-ai__topbar-link",
                    href: MCP_DOCS_URL,
                    target: "_blank",
                    rel: "noopener noreferrer",
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" }
                        polyline { points: "15 3 21 3 21 9" }
                        line {
                            x1: "10",
                            x2: "21",
                            y1: "14",
                            y2: "3",
                        }
                    }
                    "{tr.mcp_docs}"
                }
            }

            // ── Page body ────────────────────────────
            main { class: "my-ai__page",

                // Hero / endpoint card
                section { class: "my-ai__hero", "data-testid": "my-ai-hero",
                    div { class: "my-ai__hero-head",
                        div { class: "my-ai__hero-crest",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.6",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M12 3 13.6 9.4 20 11l-6.4 1.6L12 19l-1.6-6.4L4 11l6.4-1.6L12 3Z" }
                                path {
                                    d: "M19 4 19.7 6.3 22 7l-2.3 0.7L19 10l-0.7-2.3L16 7l2.3-0.7L19 4Z",
                                    opacity: "0.7",
                                    stroke_width: "1.2",
                                }
                            }
                        }
                        div { class: "my-ai__hero-head-body",
                            div { class: "my-ai__hero-eyebrow",
                                strong { "{tr.hero_eyebrow_label}" }
                                " · {tr.hero_eyebrow_transport}"
                            }
                            h1 { class: "my-ai__hero-title", "{tr.hero_title}" }
                            p { class: "my-ai__hero-sub", "{tr.hero_sub}" }
                        }
                    }

                    // URL row — render different inner content depending on
                    // whether the raw token is available right now.
                    div { class: "my-ai__url",
                        span { class: "my-ai__url-label",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                rect {
                                    height: "11",
                                    rx: "2",
                                    width: "18",
                                    x: "3",
                                    y: "11",
                                }
                                path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
                            }
                            "{tr.endpoint_label}"
                        }
                        if mcp_url.is_some() {
                            input {
                                class: "my-ai__url-value",
                                "data-testid": "my-ai-endpoint-url",
                                readonly: true,
                                value: "{display_url}",
                            }
                            button {
                                class: "my-ai__url-copy",
                                "data-copied": url_copied(),
                                "data-testid": "my-ai-copy-url",
                                onclick: on_copy_url,
                                svg {
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    rect {
                                        height: "13",
                                        rx: "2",
                                        width: "13",
                                        x: "9",
                                        y: "9",
                                    }
                                    path { d: "M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" }
                                }
                                if url_copied() {
                                    "{tr.btn_copied}"
                                } else {
                                    "{tr.btn_copy_url}"
                                }
                            }
                        } else {
                            div { class: "my-ai__url-value my-ai__url-value--placeholder",
                                if has_secret {
                                    "{tr.placeholder_existing}"
                                } else {
                                    "{tr.placeholder_none}"
                                }
                            }
                        }
                    }

                    // Heads-up note
                    div { class: "my-ai__note",
                        svg {
                            fill: "none",
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            view_box: "0 0 24 24",
                            xmlns: "http://www.w3.org/2000/svg",
                            circle { cx: "12", cy: "12", r: "10" }
                            line {
                                x1: "12",
                                x2: "12",
                                y1: "8",
                                y2: "12",
                            }
                            line {
                                x1: "12",
                                x2: "12.01",
                                y1: "16",
                                y2: "16",
                            }
                        }
                        div {
                            strong { "{tr.note_heads_up}" }
                            " {tr.note_body}"
                        }
                    }

                    // Error (if any)
                    if let Some(msg) = error_message() {
                        div { class: "my-ai__error", "{msg}" }
                    }

                    // Actions
                    div { class: "my-ai__actions",
                        button {
                            class: "my-ai__btn my-ai__btn--primary",
                            "data-testid": "my-ai-generate",
                            disabled: generating(),
                            onclick: on_generate,
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                polyline { points: "23 4 23 10 17 10" }
                                polyline { points: "1 20 1 14 7 14" }
                                path { d: "M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" }
                            }
                            if generating() {
                                "{tr.btn_generating}"
                            } else if has_secret {
                                "{tr.btn_regenerate}"
                            } else {
                                "{tr.btn_generate}"
                            }
                        }
                        span { class: "my-ai__actions-hint", "{tr.actions_hint}" }
                    }
                }

                // Capabilities
                div { class: "my-ai__section-head",
                    div { class: "my-ai__section-title", "{tr.caps_section_title}" }
                    div { class: "my-ai__section-sub", "{tr.caps_section_sub}" }
                }
                section { class: "my-ai__caps",
                    CapPill {
                        flavor: "cyan",
                        label: tr.cap_posts_label.to_string(),
                        value: tr.cap_posts_value.to_string(),
                        children: rsx! {
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.8",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                            }
                        },
                    }
                    CapPill {
                        flavor: "gold",
                        label: tr.cap_polls_label.to_string(),
                        value: tr.cap_polls_value.to_string(),
                        children: rsx! {
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.8",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M3 3v18h18" }
                                path { d: "M7 14l4-4 4 4 5-5" }
                            }
                        },
                    }
                    CapPill {
                        flavor: "violet",
                        label: tr.cap_spaces_label.to_string(),
                        value: tr.cap_spaces_value.to_string(),
                        children: rsx! {
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.8",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" }
                                circle { cx: "9", cy: "7", r: "4" }
                                path { d: "M23 21v-2a4 4 0 0 0-3-3.87" }
                                path { d: "M16 3.13a4 4 0 0 1 0 7.75" }
                            }
                        },
                    }
                    CapPill {
                        flavor: "teal",
                        label: tr.cap_discussions_label.to_string(),
                        value: tr.cap_discussions_value.to_string(),
                        children: rsx! {
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.8",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
                                line {
                                    x1: "9",
                                    x2: "15",
                                    y1: "10",
                                    y2: "10",
                                }
                            }
                        },
                    }
                    CapPill {
                        flavor: "pink",
                        label: tr.cap_inbox_label.to_string(),
                        value: tr.cap_inbox_value.to_string(),
                        children: rsx! {
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "1.8",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                path { d: "M22 12h-4l-3 9L9 3l-3 9H2" }
                            }
                        },
                    }
                }

                // Setup guide
                div { class: "my-ai__section-head",
                    div { class: "my-ai__section-title", "{tr.guide_section_title}" }
                    div { class: "my-ai__section-sub", "{tr.guide_section_sub}" }
                }

                section { class: "my-ai__guide",
                    div { class: "my-ai__guide-tabs", role: "tablist",
                        GuideTabButton {
                            active_tab,
                            tab: GuideTab::ClaudeCode,
                            label: tr.tab_claude_code.to_string(),
                            test_id: "my-ai-tab-claude-code".to_string(),
                            children: rsx! {
                                svg {
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    polyline { points: "16 18 22 12 16 6" }
                                    polyline { points: "8 6 2 12 8 18" }
                                }
                            },
                        }
                        GuideTabButton {
                            active_tab,
                            tab: GuideTab::ClaudeDesktop,
                            label: tr.tab_claude_desktop.to_string(),
                            test_id: "my-ai-tab-claude-desktop".to_string(),
                            children: rsx! {
                                svg {
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    rect {
                                        height: "14",
                                        rx: "2",
                                        width: "20",
                                        x: "2",
                                        y: "3",
                                    }
                                    line {
                                        x1: "8",
                                        x2: "16",
                                        y1: "21",
                                        y2: "21",
                                    }
                                    line {
                                        x1: "12",
                                        x2: "12",
                                        y1: "17",
                                        y2: "21",
                                    }
                                }
                            },
                        }
                        GuideTabButton {
                            active_tab,
                            tab: GuideTab::Cursor,
                            label: tr.tab_cursor.to_string(),
                            test_id: "my-ai-tab-cursor".to_string(),
                            children: rsx! {
                                svg {
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path { d: "M2 2l8 18 2-8 8-2L2 2z" }
                                }
                            },
                        }
                        GuideTabButton {
                            active_tab,
                            tab: GuideTab::Generic,
                            label: tr.tab_generic.to_string(),
                            test_id: "my-ai-tab-generic".to_string(),
                            children: rsx! {
                                svg {
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    circle { cx: "12", cy: "12", r: "10" }
                                    path { d: "M12 6v6l4 2" }
                                }
                            },
                        }
                    }

                    // Active panel content. The display_url is woven in so
                    // users can copy a working command after generating.
                    {render_panel(active_tab(), &display_url, &tr)}
                }
            }
        }
    }
}

#[component]
fn CapPill(flavor: String, label: String, value: String, children: Element) -> Element {
    let class = format!("my-ai__cap my-ai__cap--{flavor}");
    rsx! {
        div { class: "{class}",
            div { class: "my-ai__cap-icon", {children} }
            div { class: "my-ai__cap-body",
                div { class: "my-ai__cap-label", "{label}" }
                div { class: "my-ai__cap-value", "{value}" }
            }
        }
    }
}

#[component]
fn GuideTabButton(
    active_tab: Signal<GuideTab>,
    tab: GuideTab,
    label: String,
    test_id: String,
    children: Element,
) -> Element {
    let mut active_tab = active_tab;
    let is_selected = active_tab() == tab;
    rsx! {
        button {
            class: "my-ai__guide-tab",
            role: "tab",
            "aria-selected": is_selected,
            "data-testid": "{test_id}",
            onclick: move |_| active_tab.set(tab),
            {children}
            "{label}"
        }
    }
}

fn render_panel(tab: GuideTab, url: &str, tr: &MyAiTranslate) -> Element {
    match tab {
        GuideTab::ClaudeCode => render_claude_code(url, tr),
        GuideTab::ClaudeDesktop => render_claude_desktop(url, tr),
        GuideTab::Cursor => render_cursor(url, tr),
        GuideTab::Generic => render_generic(url, tr),
    }
}

fn render_claude_code(url: &str, tr: &MyAiTranslate) -> Element {
    let cmd = format!("claude mcp add --transport http ratel \\\n  {url}");
    rsx! {
        div { class: "my-ai__guide-panel",
            p { class: "my-ai__lede",
                strong { "{tr.cc_lede_prefix}" }
                " {tr.cc_lede_body}"
            }
            div { class: "my-ai__steps",
                Step {
                    number: 1,
                    title: tr.cc_step1_title.to_string(),
                    hint: Some(tr.cc_step1_hint.to_string()),
                    code: Some(("terminal".to_string(), cmd)),
                }
                Step {
                    number: 2,
                    title: tr.cc_step2_title.to_string(),
                    hint: Some(tr.cc_step2_hint.to_string()),
                    code: None,
                }
                Step {
                    number: 3,
                    title: tr.cc_step3_title.to_string(),
                    hint: Some(tr.cc_step3_hint.to_string()),
                    code: None,
                }
            }
            VerifyCard {
                title: tr.cc_verify_title.to_string(),
                hint: tr.cc_verify_hint.to_string(),
            }
        }
    }
}

fn render_claude_desktop(url: &str, tr: &MyAiTranslate) -> Element {
    let snippet = format!(
        "{{\n  \"mcpServers\": {{\n    \"ratel\": {{\n      \"transport\": \"http\",\n      \"url\": \"{url}\"\n    }}\n  }}\n}}"
    );
    rsx! {
        div { class: "my-ai__guide-panel",
            p { class: "my-ai__lede", "{tr.cd_lede}" }
            div { class: "my-ai__steps",
                Step {
                    number: 1,
                    title: tr.cd_step1_title.to_string(),
                    hint: Some(
                        format!(
                            "{}: ~/Library/Application Support/Claude/claude_desktop_config.json — {}: %APPDATA%\\Claude\\claude_desktop_config.json",
                            tr.cd_step1_macos,
                            tr.cd_step1_windows,
                        ),
                    ),
                    code: None,
                }
                Step {
                    number: 2,
                    title: tr.cd_step2_title.to_string(),
                    hint: Some(tr.cd_step2_hint.to_string()),
                    code: Some(("claude_desktop_config.json".to_string(), snippet)),
                }
                Step {
                    number: 3,
                    title: tr.cd_step3_title.to_string(),
                    hint: Some(tr.cd_step3_hint.to_string()),
                    code: None,
                }
            }
        }
    }
}

fn render_cursor(url: &str, tr: &MyAiTranslate) -> Element {
    let snippet = format!(
        "{{\n  \"mcpServers\": {{\n    \"ratel\": {{\n      \"url\": \"{url}\"\n    }}\n  }}\n}}"
    );
    rsx! {
        div { class: "my-ai__guide-panel",
            p { class: "my-ai__lede", "{tr.cur_lede}" }
            div { class: "my-ai__steps",
                Step {
                    number: 1,
                    title: tr.cur_step1_title.to_string(),
                    hint: Some(tr.cur_step1_hint.to_string()),
                    code: None,
                }
                Step {
                    number: 2,
                    title: tr.cur_step2_title.to_string(),
                    hint: Some(tr.cur_step2_hint.to_string()),
                    code: Some(("~/.cursor/mcp.json".to_string(), snippet)),
                }
                Step {
                    number: 3,
                    title: tr.cur_step3_title.to_string(),
                    hint: Some(tr.cur_step3_hint.to_string()),
                    code: None,
                }
            }
        }
    }
}

fn render_generic(url: &str, tr: &MyAiTranslate) -> Element {
    let cmd = format!(
        "curl -X POST {url} \\\n  -H \"Content-Type: application/json\" \\\n  -H \"Accept: application/json, text/event-stream\" \\\n  -d '{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/list\"}}'"
    );
    rsx! {
        div { class: "my-ai__guide-panel",
            p { class: "my-ai__lede", "{tr.gen_lede}" }
            div { class: "my-ai__steps",
                Step {
                    number: 1,
                    title: tr.gen_step1_title.to_string(),
                    hint: Some(tr.gen_step1_hint.to_string()),
                    code: Some(("terminal".to_string(), cmd)),
                }
                Step {
                    number: 2,
                    title: tr.gen_step2_title.to_string(),
                    hint: Some(tr.gen_step2_hint.to_string()),
                    code: None,
                }
            }
        }
    }
}

#[component]
fn Step(
    number: u32,
    title: String,
    hint: Option<String>,
    code: Option<(String, String)>,
) -> Element {
    rsx! {
        div { class: "my-ai__step",
            div { class: "my-ai__step-num", "{number}" }
            div { class: "my-ai__step-body",
                div { class: "my-ai__step-title", "{title}" }
                if let Some(hint_text) = hint {
                    div { class: "my-ai__step-hint", "{hint_text}" }
                }
                if let Some((path, snippet)) = code {
                    CodeBlock { path, snippet }
                }
            }
        }
    }
}

#[component]
fn CodeBlock(path: String, snippet: String) -> Element {
    // `copied` is only mutated under the `web` feature where clipboard
    // access is available; under `server` builds the closure body is
    // empty so the binding stays unused but must remain `Signal` typed
    // for the RSX read below.
    #[allow(unused_mut)]
    let mut copied = use_signal(|| false);
    let snippet_for_copy = snippet.clone();

    let on_copy = move |_: MouseEvent| {
        let snippet = snippet_for_copy.clone();
        #[cfg(feature = "web")]
        {
            spawn(async move {
                if let Some(window) = web_sys::window() {
                    let clipboard = window.navigator().clipboard();
                    let _ = wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&snippet))
                        .await;
                    copied.set(true);
                    sleep(std::time::Duration::from_millis(1600)).await;
                    copied.set(false);
                }
            });
        }
        #[cfg(not(feature = "web"))]
        {
            let _ = snippet;
        }
    };

    rsx! {
        div { class: "my-ai__code",
            div { class: "my-ai__code-head",
                span { class: "my-ai__code-path",
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        path { d: "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" }
                        polyline { points: "14 2 14 8 20 8" }
                    }
                    "{path}"
                }
                button {
                    class: "my-ai__code-copy",
                    "data-copied": copied(),
                    onclick: on_copy,
                    svg {
                        fill: "none",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2",
                        view_box: "0 0 24 24",
                        xmlns: "http://www.w3.org/2000/svg",
                        rect {
                            height: "13",
                            rx: "2",
                            width: "13",
                            x: "9",
                            y: "9",
                        }
                        path { d: "M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" }
                    }
                    if copied() {
                        "Copied"
                    } else {
                        "Copy"
                    }
                }
            }
            pre { "{snippet}" }
        }
    }
}

#[component]
fn VerifyCard(title: String, hint: String) -> Element {
    rsx! {
        div { class: "my-ai__verify",
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                stroke_width: "2",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                polyline { points: "22 4 12 14.01 9 11.01" }
            }
            div {
                div { class: "my-ai__verify-title", "{title}" }
                div { class: "my-ai__verify-hint", "{hint}" }
            }
        }
    }
}
