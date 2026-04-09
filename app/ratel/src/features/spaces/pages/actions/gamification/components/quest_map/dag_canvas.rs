use crate::common::*;
use crate::features::spaces::pages::actions::gamification::types::{QuestNodeStatus, QuestNodeView};

/// SVG connector canvas for the Quest Map DAG.
///
/// Renders Bezier curves between parent and child quest nodes to visualise
/// the dependency graph. The actual node tiles are rendered separately in
/// `ChapterSection`; this component only produces the SVG layer.
///
/// Layout:
/// - Nodes are arranged in a topological-depth × sibling-index grid.
/// - Each cell is 150 px wide and 100 px tall.
/// - Curves are drawn from the bottom-centre of a parent node to the
///   top-centre of each dependent child node.
/// - Stroke colour encodes status: gold (cleared), teal/accent (active),
///   grey dashed (locked / role-gated).
#[component]
pub fn DagCanvas(nodes: Vec<QuestNodeView>) -> Element {
    if nodes.is_empty() || nodes.iter().all(|n| n.depends_on.is_empty()) {
        return rsx! {
            div {}
        };
    }

    // ── Layout: assign row (depth) and column per node ─────────────────────
    // Depth = max depth of any parent + 1 (0 for root nodes).
    use std::collections::HashMap;

    let node_ids: HashMap<String, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.id.clone(), i))
        .collect();

    let mut depth: Vec<i32> = vec![-1; nodes.len()];

    // Iterative topo-sort inspired depth assignment (up to 10 passes for cycles).
    for _ in 0..10 {
        for (i, node) in nodes.iter().enumerate() {
            if node.depends_on.is_empty() {
                depth[i] = 0;
            } else {
                let parent_depth = node
                    .depends_on
                    .iter()
                    .filter_map(|dep_id| node_ids.get(dep_id))
                    .map(|&idx| depth[idx])
                    .max()
                    .unwrap_or(-1);
                if parent_depth >= 0 {
                    depth[i] = parent_depth + 1;
                }
            }
        }
    }

    // Column per node: index within its depth layer.
    let mut col_counter: HashMap<i32, i32> = HashMap::new();
    let mut col: Vec<i32> = vec![0; nodes.len()];
    for (i, &d) in depth.iter().enumerate() {
        if d >= 0 {
            let c = col_counter.entry(d).or_insert(0);
            col[i] = *c;
            *c += 1;
        }
    }

    const CELL_W: i32 = 150;
    const CELL_H: i32 = 100;
    const NODE_W: i32 = 120;
    const NODE_H: i32 = 70;

    let max_depth = depth.iter().copied().max().unwrap_or(0);
    let max_cols = col_counter.values().copied().max().unwrap_or(1);
    let svg_h = ((max_depth + 1) * CELL_H + 20).max(40);
    let svg_w = (max_cols * CELL_W).max(CELL_W);

    // Build path data for SVG rendering.
    struct PathData {
        key: String,
        d: String,
        stroke: &'static str,
        dash: &'static str,
    }

    let mut path_data: Vec<PathData> = Vec::new();

    for (child_idx, node) in nodes.iter().enumerate() {
        let child_d = depth[child_idx];
        let child_c = col[child_idx];
        if child_d < 0 {
            continue;
        }
        let cx = child_c * CELL_W + CELL_W / 2;
        let cy = child_d * CELL_H;

        for dep_id in &node.depends_on {
            let Some(&par_idx) = node_ids.get(dep_id.as_str()) else {
                continue;
            };
            let par_d = depth[par_idx];
            let par_c = col[par_idx];
            if par_d < 0 {
                continue;
            }

            let px = par_c * CELL_W + CELL_W / 2;
            let py = par_d * CELL_H + NODE_H;

            // Bezier control points (vertical S-curve).
            let mid_y = (py + cy) / 2;
            let d = format!("M {px} {py} C {px} {mid_y}, {cx} {mid_y}, {cx} {cy}");

            // Stroke style based on child node status.
            let (stroke, dash) = match node.status {
                QuestNodeStatus::Cleared => ("#fcb300", "none"),
                QuestNodeStatus::Active => ("#6eedd8", "none"),
                _ => ("#666666", "4 4"),
            };

            path_data.push(PathData {
                key: format!("{}-{}", dep_id, node.id),
                d,
                stroke,
                dash,
            });
        }
    }

    rsx! {
        svg {
            width: "100%",
            height: "{svg_h}px",
            view_box: "0 0 {svg_w} {svg_h}",
            preserve_aspect_ratio: "xMidYMid meet",
            class: "overflow-visible pointer-events-none",
            "data-testid": "dag-canvas",

            for p in path_data {
                path {
                    key: "{p.key}",
                    d: "{p.d}",
                    stroke: "{p.stroke}",
                    stroke_width: "2",
                    stroke_dasharray: "{p.dash}",
                    fill: "none",
                    opacity: "0.6",
                }
            }
        }
    }
}
