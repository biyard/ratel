use bdk::prelude::*;
use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::js_sys::eval;
use web_sys::{Event, HtmlElement, window};

#[component]
pub fn RichText(
    #[props(default = "rich-text".to_string())] id: String,
    content: String,
    onchange: EventHandler<String>,

    #[props(default = false)] change_location: bool,
    #[props(default = false)] remove_border: bool,
    #[props(default = "".to_string())] placeholder: String,

    #[props(default = None)] send_button: Option<Element>,
) -> Element {
    let mut closure_ref = use_signal(|| None as Option<Closure<dyn FnMut(web_sys::Event)>>);

    use_effect({
        let id = id.clone();
        let onchange = onchange.clone();
        let change_location = change_location;

        move || {
            let event_name = format!("content-updated-{}", id);

            let init_js = format!(
                r#"
                (function tryInit() {{
                    let editor = document.getElementById("{id}");
                    if (editor && window.Quill && !editor.__quill) {{
                        const parent = editor.parentElement;

                        editor.__quill = new Quill(editor, {{ theme: "snow", placeholder: "{placeholder}", modules: {{
                            toolbar: {{
                                container: [
                                    [{{ 'header': [1, 2, 3, 4, 5, 6, false] }}],
                                    ['bold', 'italic', 'underline', 'strike'],
                                    [{{ 'color': [] }}, {{ 'background': [] }}],
                                    [{{ 'list': 'ordered'}}, {{ 'list': 'bullet' }}],
                                    [{{ 'indent': '-1'}}, {{ 'indent': '+1' }}],
                                    [{{ 'direction': 'rtl' }}],
                                    [{{ 'align': [] }}],
                                    ['image'],
                                    ['clean']
                                ],
                                handlers: {{
                                    image: function() {{
                                        let fileInput = document.getElementById("file-input-{id}");
                                        if (!fileInput) {{
                                            fileInput = document.createElement("input");
                                            fileInput.setAttribute("type", "file");
                                            fileInput.setAttribute("id", "file-input-{id}");
                                            fileInput.setAttribute("accept", "image/*");
                                            fileInput.style.display = "none";
                                            document.body.appendChild(fileInput);
                                        }}
                                        fileInput.click();
                                    }}
                                }}
                            }}                       
                        }} }});

                        // move toolbar to custom slot
                        setTimeout(() => {{
                            const toolbar = parent.querySelector('.ql-toolbar');
                            const slot = document.getElementById("{id}_toolbar_slot");
                            if (toolbar && slot) {{
                                slot.appendChild(toolbar);
                            }}
                        }}, 100);

                        setTimeout(() => {{
                            const styleId = "rich-text-toolbar-style";
                            if (!document.getElementById(styleId)) {{
                                const style = document.createElement("style");
                                style.id = styleId;
                                style.innerHTML = `
                            .ql-toolbar {{
                                border: none !important;
                            }}
                            .ql-toolbar button svg,
                            .ql-toolbar button svg * {{
                                stroke: #737373 !important;
                            }}
                            .ql-toolbar button.ql-active svg,
                            .ql-toolbar button.ql-active svg * {{
                                stroke: #facc15 !important;
                            }}
                            .ql-toolbar button:hover svg,
                            .ql-toolbar button:hover svg * {{
                                stroke: #facc15 !important;
                            }}
                            .ql-toolbar .ql-picker-label {{
                                stroke: #737373 !important;
                            }}
                            .ql-toolbar .ql-picker-label.ql-active {{
                                stroke: #facc15 !important;
                            }}
                            .ql-toolbar .ql-picker-options {{
                                background-color: #222 !important;
                            }}
                        `;
                                document.head.appendChild(style);
                            }}
                        }}, 0);

                        const container = parent.querySelector('.ql-container');

                        {move_toolbar_js}
                        {remove_border_js}
                        {placeholder_style_css}

                        editor.__quill.on('text-change', function() {{
                            document.dispatchEvent(new CustomEvent("{event_name}"));
                        }});
                    }} else {{
                        setTimeout(tryInit, 200);
                    }}
                }})();
                "#,
                id = id,
                event_name = event_name,
                placeholder = placeholder,
                move_toolbar_js = if change_location {
                    r#"
                        if (container) {
                            const parent = container.parentElement;
                            container.remove();
                            parent.insertBefore(container, parent.firstChild);
                        }
                    "#
                } else {
                    ""
                },
                remove_border_js = if remove_border {
                    r#"
                        setTimeout(() => {
                            const container = parent.querySelector('.ql-container');
                            if (container) {
                                container.style.setProperty("border", "none", "important");
                                container.style.setProperty("border-top", "none", "important");
                                container.style.setProperty("border-bottom", "none", "important");
                            }
                            if (editor) {
                                editor.style.setProperty("border", "none", "important");
                                editor.style.setProperty("outline", "none", "important");
                            }
                        }, 0);
                    "#
                } else {
                    ""
                },
                placeholder_style_css = r#"
                    const styleId = "rich-text-placeholder-style";
                    if (!document.getElementById(styleId)) {
                        const style = document.createElement("style");
                        style.id = styleId;
                        style.innerHTML = `
                            .ql-editor::before {
                                color: #525252 !important;
                            }
                        `;
                        document.head.appendChild(style);
                    }
                "#
            );

            let _ = eval(&init_js);

            let id_cloned = id.clone();
            let onchange_cloned = onchange.clone();
            let closure = Closure::wrap(Box::new(move |_event: Event| {
                if let Some(editor) = window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .get_element_by_id(&id_cloned)
                {
                    if let Ok(Some(ql_editor)) = editor
                        .dyn_ref::<HtmlElement>()
                        .unwrap()
                        .query_selector(".ql-editor")
                    {
                        let html = ql_editor.inner_html();
                        onchange_cloned.call(html);
                    }
                }
            }) as Box<dyn FnMut(_)>);

            window()
                .unwrap()
                .document()
                .unwrap()
                .add_event_listener_with_callback(&event_name, closure.as_ref().unchecked_ref())
                .unwrap();

            closure_ref.set(Some(closure));

            let _ = move || {
                if let Some(cleanup) = closure_ref.take() {
                    let _ = window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .remove_event_listener_with_callback(
                            &event_name,
                            cleanup.as_ref().unchecked_ref(),
                        );

                    closure_ref.set(None);
                }
            };
        }
    });

    use_effect({
        let id = id.clone();
        let content = content.clone();
        move || {
            let sync_js = format!(
                r#"
                (function syncContent() {{
                    let editor = document.getElementById("{id}");
                    if (editor && editor.__quill) {{
                        let current = editor.__quill.root.innerHTML;
                        let next = `{content}`;
                        if (current !== next) {{
                            editor.__quill.clipboard.dangerouslyPasteHTML(next);
                        }}
                    }}
                }})();
                "#
            );
            let _ = eval(&sync_js);
        }
    });

    rsx! {
        link {
            rel: "stylesheet",
            href: "https://cdn.jsdelivr.net/npm/quill@2.0.0-dev.4/dist/quill.snow.css",
        }
        script { src: "https://cdn.jsdelivr.net/npm/quill@2.0.0-dev.4/dist/quill.min.js" }

        div { class: "flex flex-col w-full h-150 justify-between items-start gap-5",
            div {
                id: format!("{}_toolbar_wrapper", id),
                class: "flex flex-row justify-between h-49 items-center w-full gap-8",

                div { id: format!("{}_toolbar_slot", id) }

                if let Some(button) = send_button {
                    {button}
                }
            }

            div {
                id: id.clone(),
                class: "rich-text-editor w-full h-[100px] overflow-y-auto border border-gray-300 rounded-md",
            }
        }

        input {
            r#type: "file",
            id: format!("file-input-{}", id),
            style: "display:none;",
            onchange: move |evt| {
                if let Some(file_engine) = evt.files() {
                    let filenames = file_engine.files();
                    tracing::debug!("filenames: {:?}", filenames);
                }
            },
        }
    }
}
