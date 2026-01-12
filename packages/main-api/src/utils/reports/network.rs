use crate::utils::reports::preprocess_korean_nouns;
use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct NetworkConfigV1 {
    // 문장 내 최소 토큰 수
    pub min_tokens_per_doc: usize,
    // 간선 최소 빈도 수 (단어쌍 빈도 수)
    pub min_edge_count: u32,
    // 상위 노드 갯수
    pub top_nodes: usize,
}

impl Default for NetworkConfigV1 {
    fn default() -> Self {
        Self {
            min_tokens_per_doc: 2,
            min_edge_count: 3,
            top_nodes: 30,
        }
    }
}

fn betweenness_centrality_undirected(adj: &[Vec<usize>]) -> Vec<f64> {
    let n = adj.len();
    let mut bc = vec![0.0f64; n];
    if n == 0 {
        return bc;
    }

    let mut stack: Vec<usize> = Vec::new();
    let mut pred: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut sigma = vec![0.0f64; n];
    let mut dist = vec![-1i32; n];
    let mut delta = vec![0.0f64; n];
    let mut q: VecDeque<usize> = VecDeque::new();

    for s in 0..n {
        stack.clear();
        for p in pred.iter_mut() {
            p.clear();
        }
        sigma.fill(0.0);
        dist.fill(-1);
        delta.fill(0.0);
        q.clear();

        sigma[s] = 1.0;
        dist[s] = 0;
        q.push_back(s);

        while let Some(v) = q.pop_front() {
            stack.push(v);
            let dv = dist[v];
            for &w in &adj[v] {
                if dist[w] < 0 {
                    dist[w] = dv + 1;
                    q.push_back(w);
                }
                if dist[w] == dv + 1 {
                    sigma[w] += sigma[v];
                    pred[w].push(v);
                }
            }
        }

        while let Some(w) = stack.pop() {
            for &v in &pred[w] {
                if sigma[w] != 0.0 {
                    delta[v] += (sigma[v] / sigma[w]) * (1.0 + delta[w]);
                }
            }
            if w != s {
                bc[w] += delta[w];
            }
        }
    }

    for v in 0..n {
        bc[v] *= 0.5;
    }

    if n > 2 {
        let scale = 2.0 / ((n as f64 - 1.0) * (n as f64 - 2.0));
        for v in 0..n {
            bc[v] *= scale;
        }
    } else {
        bc.fill(0.0);
    }

    bc
}

// TODO: check network logic
pub fn run_network(comments: &[String], cfg: NetworkConfigV1) -> crate::Result<NetworkGraph> {
    let docs = comments
        .iter()
        .map(|c| preprocess_korean_nouns(c))
        .filter(|t| t.len() >= cfg.min_tokens_per_doc)
        .collect::<Vec<_>>();

    let mut edge_counts: HashMap<(String, String), u32> = HashMap::new();

    for doc in &docs {
        let uniq: HashSet<&str> = doc.iter().map(|s| s.as_str()).collect();
        let mut words: Vec<&str> = uniq.into_iter().collect();
        words.sort_unstable();

        for a in 0..words.len() {
            for b in (a + 1)..words.len() {
                let w1 = words[a];
                let w2 = words[b];
                *edge_counts
                    .entry((w1.to_string(), w2.to_string()))
                    .or_insert(0) += 1;
            }
        }
    }

    let filtered_edges: Vec<(String, String, u32)> = edge_counts
        .into_iter()
        .filter_map(|((a, b), c)| {
            if c >= cfg.min_edge_count {
                Some((a, b, c))
            } else {
                None
            }
        })
        .collect();

    if filtered_edges.is_empty() {
        return Ok(NetworkGraph::default());
    }

    let mut node2id: HashMap<String, usize> = HashMap::new();
    let mut id2node: Vec<String> = Vec::new();

    let mut get_id = |s: &str| {
        if let Some(&id) = node2id.get(s) {
            return id;
        }
        let id = id2node.len();
        node2id.insert(s.to_string(), id);
        id2node.push(s.to_string());
        id
    };

    for (a, b, _) in &filtered_edges {
        let _ = get_id(a);
        let _ = get_id(b);
    }

    let n = id2node.len();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];

    for (a, b, _) in &filtered_edges {
        let ia = node2id[a];
        let ib = node2id[b];
        adj[ia].push(ib);
        adj[ib].push(ia);
    }

    for v in 0..n {
        adj[v].sort_unstable();
        adj[v].dedup();
    }

    let denom = if n > 1 { n as f64 - 1.0 } else { 1.0 };
    let degree: Vec<f64> = (0..n).map(|i| adj[i].len() as f64 / denom).collect();
    let betweenness = betweenness_centrality_undirected(&adj);

    let mut ranked: Vec<usize> = (0..n).collect();
    ranked.sort_by(|&a, &b| degree[b].partial_cmp(&degree[a]).unwrap());

    let top = ranked.into_iter().take(cfg.top_nodes).collect::<Vec<_>>();
    let top_set: HashSet<usize> = top.iter().copied().collect();

    let mut nodes = Vec::with_capacity(top.len());
    for i in &top {
        nodes.push(NetworkCentralityRow {
            node: id2node[*i].clone(),
            degree_centrality: degree[*i],
            betweenness_centrality: betweenness[*i],
        });
    }

    let mut edges = Vec::new();
    for (a, b, w) in &filtered_edges {
        let ia = node2id[a];
        let ib = node2id[b];

        if !top_set.contains(&ia) || !top_set.contains(&ib) {
            continue;
        }

        edges.push(NetworkEdgeRow {
            source: id2node[ia].clone(),
            target: id2node[ib].clone(),
            weight: *w,
        });
    }

    edges.sort_by(|x, y| y.weight.cmp(&x.weight));

    Ok(NetworkGraph { nodes, edges })
}
