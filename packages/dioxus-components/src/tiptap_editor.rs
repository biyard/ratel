use dioxus::prelude::*;
use web_sys::wasm_bindgen::JsCast;

pub const TIPTAP_EDITOR_JS: Asset = asset!("/assets/tiptap-editor.js", AssetOptions::js());

#[derive(Debug, Props, Clone, PartialEq)]
pub struct TiptapEditorProps {
    #[props(default)]
    pub content: String,
    #[props(default = true)]
    pub editable: bool,
    #[props(default = "Type here...".to_string())]
    pub placeholder: String,
    #[props(default)]
    pub on_content_change: Option<EventHandler<String>>,
    #[props(default)]
    pub class: String,
}

#[component]
pub fn TiptapEditor(props: TiptapEditorProps) -> Element {
    rsx! {
        Fragment {
            document::Script { src: TIPTAP_EDITOR_JS }
            tiptap-editor {
                class: "{props.class}",
                content: "{props.content}",
                editable: if props.editable { "true" } else { "false" },
                placeholder: "{props.placeholder}",
                onchange: move |evt| {
                    if let Some(raw_event) = evt.data().downcast::<web_sys::Event>() {
                        if let Some(custom_event) = raw_event.dyn_ref::<web_sys::CustomEvent>() {
                            if let Some(val) = custom_event.detail().as_string() {
                                if let Some(handler) = &props.on_content_change {
                                    handler.call(val);
                                }
                            }
                        }
                    }
                },
            }

        }
    }
}
