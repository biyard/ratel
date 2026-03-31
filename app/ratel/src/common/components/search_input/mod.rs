use crate::common::*;

/// A reusable tag-input component that displays selected items as badges
/// inside the input area, with a text input for searching/adding new values.
///
/// Features:
/// - Badges with X buttons rendered inside the input box
/// - Dropdown suggestions from a provided list
/// - Enter or comma to add a tag
/// - Backspace on empty input removes the last tag
/// - `on_add` / `on_remove` callbacks for parent state management
/// - Optional `on_create_new` for creating items not in the suggestion list
#[component]
pub fn SearchInput(
    /// Currently selected tags to display as badges
    #[props(default)]
    tags: Vec<String>,
    /// All available suggestions for the dropdown
    #[props(default)]
    suggestions: Vec<String>,
    /// Placeholder text for the input
    #[props(default)]
    placeholder: String,
    /// Label for the "create new" button (e.g. "Create")
    #[props(default)]
    create_label: String,
    /// Optional extra class for the outer container
    #[props(default)]
    class: String,
    /// Called when a new tag should be added (provides the tag string).
    /// Parent is responsible for dedup and updating tags.
    on_add: Option<EventHandler<String>>,
    /// Called when a tag is removed (provides the tag string)
    on_remove: Option<EventHandler<String>>,
    /// Called when user wants to create a new item not in the suggestions.
    /// Receives the typed input value. If not set, no "create" button is shown.
    on_create_new: Option<EventHandler<String>>,
    /// Whether a create-new operation is in progress (shows loading)
    #[props(default)]
    creating: bool,
    /// data-testid for the outer container
    #[props(default)]
    data_testid: Option<String>,
) -> Element {
    let mut input_value = use_signal(|| String::new());
    let mut show_dropdown = use_signal(|| false);

    let filtered_suggestions: Vec<String> = suggestions
        .iter()
        .filter(|s| {
            let input_lower = input_value().to_lowercase();
            !input_lower.trim().is_empty()
                && s.to_lowercase().contains(&input_lower)
                && !tags.iter().any(|t| t.to_lowercase() == s.to_lowercase())
        })
        .cloned()
        .collect();

    let input_trimmed = input_value().trim().to_string();
    let show_create_button = on_create_new.is_some()
        && !input_trimmed.is_empty()
        && !suggestions
            .iter()
            .any(|c| c.to_lowercase() == input_trimmed.to_lowercase());

    let has_dropdown_content = !filtered_suggestions.is_empty() || show_create_button;

    rsx! {
        div {
            class: "relative w-full {class}",
            "data-testid": data_testid.clone().unwrap_or_default(),
            onfocusout: move |_| {
                spawn(async move {
                    crate::common::utils::time::sleep(std::time::Duration::from_millis(150))
                        .await;
                    show_dropdown.set(false);
                });
            },

            // Main input-like container with badges + input inside
            div {
                class: "flex flex-wrap gap-2 items-center px-3 py-2 w-full min-h-[44px] rounded-[10px] border bg-input-box-bg border-input-box-border focus-within:border-ring focus-within:ring-ring/50 focus-within:ring-[1px] transition-[color,box-shadow]",

                // Render selected tags as badges inside the container
                for tag in tags.iter().cloned() {
                    div {
                        key: "{tag}",
                        class: "flex items-center",
                        "data-testid": "search-input-tag",
                        Badge {
                            color: BadgeColor::Blue,
                            size: BadgeSize::Small,
                            variant: BadgeVariant::Rounded,
                            class: "flex items-center gap-1 cursor-default",
                            span { "{tag}" }
                            span {
                                class: "ml-1 cursor-pointer opacity-60 hover:opacity-100",
                                "data-testid": "search-input-tag-remove",
                                onclick: {
                                    let tag = tag.clone();
                                    move |e: MouseEvent| {
                                        e.stop_propagation();
                                        if let Some(ref on_remove) = on_remove {
                                            on_remove.call(tag.clone());
                                        }
                                    }
                                },
                                "\u{2715}"
                            }
                        }
                    }
                }

                // The actual text input (plain, no extra border)
                input {
                    class: "flex-1 min-w-[80px] text-base font-light border-none outline-none md:text-sm bg-transparent text-text-primary placeholder:text-muted-foreground",
                    placeholder: if tags.is_empty() { placeholder.clone() } else { String::new() },
                    value: "{input_value}",
                    "data-testid": "search-input-field",
                    oninput: move |e: Event<FormData>| {
                        let val = e.value();
                        if val.ends_with(',') {
                            let trimmed = val.trim_end_matches(',').trim().to_string();
                            if !trimmed.is_empty() {
                                if let Some(ref on_add) = on_add {
                                    on_add.call(trimmed);
                                }
                                input_value.set(String::new());
                            }
                            show_dropdown.set(false);
                        } else {
                            input_value.set(val);
                            show_dropdown.set(true);
                        }
                    },
                    onkeydown: move |e: KeyboardEvent| {
                        if e.key() == Key::Enter {
                            let trimmed = input_value().trim().to_string();
                            if !trimmed.is_empty() {
                                if let Some(ref on_add) = on_add {
                                    on_add.call(trimmed);
                                }
                                input_value.set(String::new());
                            }
                            show_dropdown.set(false);
                        } else if e.key() == Key::Backspace && input_value().is_empty() {
                            if let Some(last) = tags.last() {
                                if let Some(ref on_remove) = on_remove {
                                    on_remove.call(last.clone());
                                }
                            }
                        }
                    },
                }
            }

            // Dropdown
            if show_dropdown() && !input_value().trim().is_empty() && has_dropdown_content {
                div {
                    class: "overflow-y-auto absolute z-20 mt-2 w-full max-h-60 rounded-md border shadow-md bg-card border-post-input-border",
                    "data-testid": "search-input-dropdown",

                    for suggestion in filtered_suggestions.iter().cloned() {
                        div {
                            key: "{suggestion}",
                            class: "py-2 px-3 text-sm cursor-pointer text-text-primary hover:bg-muted",
                            onclick: {
                                let suggestion = suggestion.clone();
                                move |_| {
                                    if let Some(ref on_add) = on_add {
                                        on_add.call(suggestion.clone());
                                    }
                                    input_value.set(String::new());
                                    show_dropdown.set(false);
                                }
                            },
                            "{suggestion}"
                        }
                    }

                    // "Create new" button
                    if show_create_button {
                        Button {
                            class: "py-2 px-3 w-full text-sm text-primary",
                            style: ButtonStyle::Text,
                            shape: ButtonShape::Square,
                            loading: creating,
                            onclick: move |_| {
                                let val = input_value().trim().to_string();
                                if let Some(ref on_create_new) = on_create_new {
                                    on_create_new.call(val);
                                }
                                input_value.set(String::new());
                                show_dropdown.set(false);
                            },
                            "{create_label} \"{input_value()}\""
                        }
                    }
                }
            }
        }
    }
}
