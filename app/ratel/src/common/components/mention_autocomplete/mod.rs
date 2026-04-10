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
    text: ReadSignal<String>,
    on_select: EventHandler<MentionInsert>,
    members: ReadSignal<Vec<MentionCandidate>>,
    children: Element,
) -> Element {
    let mut show_dropdown = use_signal(|| false);
    let mut query = use_signal(String::new);
    let mut at_position = use_signal(|| 0usize);
    let mut selected_index = use_signal(|| 0usize);

    use_effect(move || {
        let val = text();
        if let Some(pos) = find_active_at(&val) {
            let after_at = &val[pos + 1..];
            let q: String = after_at
                .chars()
                .take_while(|c| !c.is_whitespace())
                .collect();
            query.set(q);
            at_position.set(pos);
            show_dropdown.set(true);
            selected_index.set(0);
        } else {
            show_dropdown.set(false);
        }
    });

    let filtered: Vec<MentionCandidate> = if show_dropdown() {
        let q = query().to_lowercase();
        members()
            .into_iter()
            .filter(|m| {
                m.display_name.to_lowercase().contains(&q) || m.username.to_lowercase().contains(&q)
            })
            .collect()
    } else {
        vec![]
    };

    let filtered_for_keydown = filtered.clone();

    rsx! {
        div {
            class: "relative",
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
                    Key::Enter => {
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
                    class: "absolute right-0 left-0 z-50 mt-1 max-h-[200px] overflow-y-auto rounded-lg shadow-lg bg-popover",
                    role: "listbox",
                    for (i , member) in filtered.iter().enumerate() {
                        {
                            let is_selected = i == selected_index();
                            let member_for_click = member.clone();
                            rsx! {
                                button {
                                    key: "{member.user_pk}",
                                    class: "flex gap-2 items-center py-1 px-3 w-full text-left transition-colors text-text-primary aria-selected:bg-hover hover:bg-hover",
                                    role: "option",
                                    "aria-selected": is_selected,
                                    onclick: move |_| {
                                        let insert = build_mention_insert(&member_for_click, at_position(), &query());
                                        on_select.call(insert);
                                        show_dropdown.set(false);
                                    },
                                    if !member.profile_url.is_empty() {
                                        img {
                                            class: "object-cover w-6 h-6 rounded",
                                            src: "{member.profile_url}",
                                            alt: "{member.display_name}",
                                        }
                                    } else {
                                        div { class: "flex justify-center items-center w-6 h-6 rounded bg-hover text-foreground-muted text-sm font-medium",
                                            "{member.display_name.chars().next().unwrap_or('?')}"
                                        }
                                    }
                                    span { class: "flex-1 text-sm font-semibold", "{member.display_name}" }
                                    if is_selected {
                                        span { class: "py-0.5 px-2 text-xs rounded border text-foreground-muted border-border",
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
