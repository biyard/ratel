use super::*;
use common::components::{TiptapEditor, Typo, Variant, Weight};

#[component]
pub fn ViewerPage(space_id: SpacePartition) -> Element {
    let mut content = use_signal(|| {
        r#"<h1>Decentralized Governance Framework</h1><p>This proposal outlines a comprehensive framework for decentralized legislative processes, enabling transparent and verifiable decision-making across distributed communities.</p><h2>Background</h2><p>Traditional legislative systems face challenges in <strong>transparency</strong>, <strong>accessibility</strong>, and <strong>accountability</strong>. By leveraging blockchain technology and decentralized identity (DID), we can build systems that address these shortcomings while maintaining the rigor of formal governance.</p><h2>Key Objectives</h2><ul><li><p>Establish a tamper-proof record of all legislative proposals and votes</p></li><li><p>Enable <em>real-time participation</em> from stakeholders regardless of geographic location</p></li><li><p>Implement role-based access control through verifiable credentials</p></li><li><p>Provide multi-language support for inclusive governance</p></li></ul><h2>Technical Architecture</h2><p>The system is composed of three layers:</p><ol><li><p><strong>Data Layer</strong> — DynamoDB-backed storage with on-chain anchoring for critical state transitions</p></li><li><p><strong>Application Layer</strong> — REST APIs built with Axum, handling authentication, proposal management, and voting logic</p></li><li><p><strong>Presentation Layer</strong> — A React-based frontend with collaborative editing powered by Tiptap and Yjs</p></li></ol><h2>Implementation Timeline</h2><table><tbody><tr><th>Phase</th><th>Description</th><th>Duration</th></tr><tr><td>Phase 1</td><td>Core infrastructure and authentication</td><td>4 weeks</td></tr><tr><td>Phase 2</td><td>Proposal creation and voting mechanisms</td><td>6 weeks</td></tr><tr><td>Phase 3</td><td>Real-time collaboration and notifications</td><td>4 weeks</td></tr><tr><td>Phase 4</td><td>DAO integration and on-chain governance</td><td>6 weeks</td></tr></tbody></table><h2>Conclusion</h2><p>This framework represents a significant step toward bridging the gap between <strong>crypto communities</strong> and <strong>policymakers</strong>. By providing tools that are both technically robust and user-friendly, we aim to make decentralized governance accessible to all participants.</p><blockquote><p>"The strength of a democracy lies not in its institutions, but in the active participation of its citizens."</p></blockquote>"#.to_string()
    });

    rsx! {
        div { class: "flex flex-col gap-4 flex-1 h-full",
            Typo { variant: Variant::H1, weight: Weight::Extrabold, "TIPTAP EDITOR" }
            TiptapEditor {
                class: "w-full h-full",
                content: content(),
                editable: true,
                placeholder: "Type here...",
                on_content_change: move |html: String| {
                    content.set(html);
                },
            }
        }
    }
}
