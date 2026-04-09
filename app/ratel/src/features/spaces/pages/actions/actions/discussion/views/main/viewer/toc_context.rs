use dioxus::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TocEntry {
    pub id: String,
    pub text: String,
    pub level: u8, // 1 | 2 | 3
}

#[derive(Clone, Copy)]
pub struct DiscussionTocContext {
    pub headings: Signal<Vec<TocEntry>>,
    pub active_id: Signal<Option<String>>,
}

pub fn use_discussion_toc_context() -> DiscussionTocContext {
    use_context()
}

impl DiscussionTocContext {
    pub fn init() -> Self {
        let ctx = Self {
            headings: Signal::new(Vec::new()),
            active_id: Signal::new(None),
        };
        use_context_provider(|| ctx);
        ctx
    }
}
