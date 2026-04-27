use crate::common::*;

#[derive(Clone, PartialEq, Debug)]
pub struct MentionCandidate {
    pub user_pk: String,
    pub display_name: String,
    pub username: String,
    pub profile_url: String,
}

#[derive(Clone, Debug)]
pub struct MentionInsert {
    pub start_offset: usize,
    pub end_offset: usize,
    pub display_text: String,
    pub display_name: String,
    pub user_pk: String,
}

#[component]
pub fn MentionAutocomplete(
    text: Signal<String>,
    on_select: EventHandler<MentionInsert>,
    members: ReadSignal<Vec<MentionCandidate>>,
    // Fires whenever the active `@` query changes. `Some("")` means the user
    // typed `@` with no characters yet (parent should prefetch or show a
    // default list); `Some("foo")` means they are searching "foo"; `None`
    // means no active mention is being composed.
    #[props(default)] on_query_change: EventHandler<Option<String>>,
    // Priority-ordered user pks (earlier = higher). Used to hoist thread
    // participants above the raw server prefix order so replying to a
    // thread shows the conversation's own voices first. Empty = unchanged.
    #[props(default)] priority_user_pks: ReadSignal<Vec<String>>,
    children: Element,
) -> Element {
    let mut show_dropdown = use_signal(|| false);
    let mut query = use_signal(String::new);
    let mut at_position = use_signal(|| 0usize);
    let mut selected_index = use_signal(|| 0usize);

    // Tracks the last value we reported upstream so we can skip redundant
    // callback calls. Read via `.peek()` to avoid re-subscribing on writes.
    let mut last_reported: Signal<Option<String>> = use_signal(|| None);

    // Recompute mention state from the current text. Called from the
    // wrapper's `oninput` (which captures bubbled input events from the
    // child textarea). Driven by the DOM event rather than `use_effect`
    // because Dioxus 0.7 does not reliably re-establish reactive
    // subscriptions on prop signals after SSR hydration — the effect can
    // silently fail to re-run on the first input post-hydration, leaving
    // the dropdown permanently dormant.
    let mut recompute = move || {
        let val = text();
        if let Some(pos) = find_active_at(&val) {
            let after_at = &val[pos + 1..];
            let q: String = after_at
                .chars()
                .take_while(|c| !c.is_whitespace())
                .collect();
            let next = Some(q.clone());
            if *last_reported.peek() != next {
                last_reported.set(next.clone());
                on_query_change.call(next);
            }
            query.set(q);
            at_position.set(pos);
            show_dropdown.set(true);
            selected_index.set(0);
        } else {
            if last_reported.peek().is_some() {
                last_reported.set(None);
                on_query_change.call(None);
            }
            show_dropdown.set(false);
        }
    };

    // Client-side prefix filter mirrors the server's prefix semantics.
    // When the dropdown consumer (e.g. space discussion) already fetches a
    // server-filtered list this is a no-op pass-through; when it feeds a
    // locally-built candidate list (e.g. post comment authors) this keeps
    // match behavior consistent across both surfaces.
    let filtered: Vec<MentionCandidate> = if show_dropdown() {
        let q = query().to_lowercase();
        let mut matches: Vec<MentionCandidate> = members()
            .into_iter()
            .filter(|m| {
                m.display_name.to_lowercase().starts_with(&q)
                    || m.username.to_lowercase().starts_with(&q)
            })
            .collect();

        // Stable-sort by priority rank so thread participants rise to the
        // top while the server's original ordering wins all ties (including
        // every non-priority candidate, which shares usize::MAX).
        let priority = priority_user_pks();
        if !priority.is_empty() {
            let rank: std::collections::HashMap<String, usize> = priority
                .iter()
                .enumerate()
                .map(|(i, pk)| (pk.clone(), i))
                .collect();
            matches.sort_by_key(|m| rank.get(&m.user_pk).copied().unwrap_or(usize::MAX));
        }
        matches
    } else {
        vec![]
    };

    let filtered_for_keydown = filtered.clone();

    rsx! {
        div {
            class: "relative",
            // Children's textarea `oninput` bubbles here. Driving the
            // mention recompute from the wrapper's bubbled event keeps the
            // logic out of `use_effect`, which is unreliable on
            // SSR-hydrated prop signals (see `recompute` definition above).
            oninput: move |_| recompute(),
            onkeydown: move |evt: KeyboardEvent| {
                if !show_dropdown() {
                    return;
                }
                match evt.key() {
                    Key::ArrowDown => {
                        evt.prevent_default();
                        let max = filtered_for_keydown.len().saturating_sub(1);
                        selected_index.set((selected_index() + 1).min(max));
                    }
                    Key::ArrowUp => {
                        evt.prevent_default();
                        selected_index.set(selected_index().saturating_sub(1));
                    }
                    Key::Enter | Key::Tab => {
                        // Leave modifier-qualified Enter/Tab for parent
                        // shortcuts (e.g. Ctrl+Enter to submit).
                        if evt.modifiers().contains(Modifiers::CONTROL)
                            || evt.modifiers().contains(Modifiers::META)
                            || evt.modifiers().contains(Modifiers::ALT)
                        {
                            return;
                        }
                        if let Some(member) = filtered_for_keydown.get(selected_index()) {
                            evt.prevent_default();
                            evt.stop_propagation();
                            let insert = build_mention_insert(member, at_position(), &query());
                            on_select.call(insert);
                            show_dropdown.set(false);
                        }
                    }
                    Key::Escape => {
                        show_dropdown.set(false);
                    }
                    _ => {}
                }
            },

            {children}

            if show_dropdown() && !filtered.is_empty() {
                div {
                    class: "overflow-y-auto absolute right-0 left-0 z-50 mt-1 rounded-lg shadow-lg max-h-[200px] bg-popover",
                    role: "listbox",
                    for (i, member) in filtered.iter().enumerate() {
                        {
                            let is_selected = i == selected_index();
                            let member_for_click = member.clone();
                            rsx! {
                                button {
                                    key: "{member.user_pk}",
                                    class: "flex gap-2 items-center py-1 px-3 w-full text-left transition-colors text-text-primary aria-selected:bg-hover hover:bg-hover",
                                    role: "option",
                                    "aria-selected": is_selected,
                                    // Preserve textarea focus across the click.
                                    onmousedown: move |evt| {
                                        evt.prevent_default();
                                    },
                                    onclick: move |_| {
                                        let insert = build_mention_insert(&member_for_click, at_position(), &query());
                                        on_select.call(insert);
                                        show_dropdown.set(false);
                                    },
                                    if !member.profile_url.is_empty() {
                                        img {
                                            class: "object-cover shrink-0 w-6 h-6 rounded",
                                            src: "{member.profile_url}",
                                            alt: "{member.display_name}",
                                        }
                                    } else {
                                        div { class: "flex shrink-0 justify-center items-center w-6 h-6 text-sm font-medium rounded bg-hover text-foreground-muted",
                                            "{member.display_name.chars().next().unwrap_or('?')}"
                                        }
                                    }
                                    // Slack-style single-line row: display
                                    // name is the primary target and wins
                                    // truncation priority; username follows
                                    // in muted text and collapses first via
                                    // min-w-0 + truncate on the name and
                                    // flex-shrink on the username span.
                                    span { class: "flex overflow-hidden flex-1 gap-2 items-baseline min-w-0",
                                        span { class: "text-sm font-semibold truncate text-text-primary", "{member.display_name}" }
                                        if !member.username.is_empty() {
                                            span { class: "text-xs truncate shrink text-foreground-muted", "@{member.username}" }
                                        }
                                    }
                                    if is_selected {
                                        span { class: "py-0.5 px-2 text-xs rounded border shrink-0 text-foreground-muted border-border",
                                            "Enter"
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

fn build_mention_insert(member: &MentionCandidate, at_pos: usize, q: &str) -> MentionInsert {
    let display_text = crate::common::utils::mention::mention_display(&member.display_name);
    MentionInsert {
        start_offset: at_pos,
        end_offset: at_pos + 1 + q.len(),
        display_text,
        display_name: member.display_name.clone(),
        user_pk: member.user_pk.clone(),
    }
}

fn find_active_at(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut i = bytes.len();
    while i > 0 {
        i -= 1;
        if bytes[i] == b'@' {
            // Skip completed mention markup like @[name](user:pk)
            if i + 1 < bytes.len() && bytes[i + 1] == b'[' {
                continue;
            }
            let after = &text[i + 1..];
            let query_part: String = after.chars().take_while(|c| !c.is_whitespace()).collect();
            // If there's whitespace after the query, the @ is no longer "active"
            if query_part.len() < after.len() {
                continue;
            }
            return Some(i);
        }
    }
    None
}
