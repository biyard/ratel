//! Word co-occurrence network for the matched-user comment corpus.
//!
//! Each document contributes co-occurrence edges between every unique
//! pair of distinct tokens it contains. After aggregation we keep the
//! top-N nodes by frequency, prune edges to those that connect two
//! kept nodes, and return both lists ready for the panel renderer.

use crate::features::spaces::pages::apps::apps::analyzes::types::{NetworkEdge, NetworkNode};
use std::collections::HashMap;
use std::collections::HashSet;

pub fn run_text_network(token_docs: &[Vec<String>], top_n_nodes: usize) -> (Vec<NetworkNode>, Vec<NetworkEdge>) {
    if token_docs.is_empty() || top_n_nodes == 0 {
        return (Vec::new(), Vec::new());
    }

    // Term frequency = number of documents containing the term.
    // Nodes are ranked by this so we never lose a term that's
    // prevalent corpus-wide just because it's rare per-doc.
    let mut node_freq: HashMap<String, u32> = HashMap::new();
    // Edge weight = number of documents in which both terms co-occur.
    let mut edge_weight: HashMap<(String, String), u32> = HashMap::new();

    for doc in token_docs {
        if doc.is_empty() {
            continue;
        }
        // Per-doc unique set so a single comment doesn't double-count
        // its term frequency or self-loop a co-occurrence.
        let unique: HashSet<&str> = doc.iter().map(|s| s.as_str()).collect();
        for tok in &unique {
            *node_freq.entry((*tok).to_string()).or_insert(0) += 1;
        }
        let mut sorted: Vec<&str> = unique.into_iter().collect();
        sorted.sort();
        for i in 0..sorted.len() {
            for j in (i + 1)..sorted.len() {
                let a = sorted[i].to_string();
                let b = sorted[j].to_string();
                *edge_weight.entry((a, b)).or_insert(0) += 1;
            }
        }
    }

    // Keep top-N nodes by frequency.
    let mut nodes: Vec<(String, u32)> = node_freq.into_iter().collect();
    nodes.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    nodes.truncate(top_n_nodes);

    let kept: HashSet<&str> = nodes.iter().map(|(t, _)| t.as_str()).collect();

    let mut edges: Vec<NetworkEdge> = edge_weight
        .into_iter()
        .filter(|((a, b), _)| kept.contains(a.as_str()) && kept.contains(b.as_str()))
        .map(|((a, b), w)| NetworkEdge {
            source: a,
            target: b,
            weight: w,
        })
        .collect();
    // Stable order — heaviest edges first so client truncation stays
    // meaningful if it ever caps the count.
    edges.sort_by(|a, b| {
        b.weight
            .cmp(&a.weight)
            .then_with(|| a.source.cmp(&b.source))
            .then_with(|| a.target.cmp(&b.target))
    });

    let nodes_out: Vec<NetworkNode> = nodes
        .into_iter()
        .map(|(term, weight)| NetworkNode { term, weight })
        .collect();

    (nodes_out, edges)
}
