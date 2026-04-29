//! LDA topic modelling via collapsed Gibbs sampling.
//!
//! Treats each comment as a document, each preprocessed noun as a
//! token. Returns one `TopicRow` per (topic × top-N keywords) pair —
//! keywords ordered by per-topic word distribution `phi` desc.
//!
//! No external ML crates: a few hundred lines of `nw` / `nd` / `nwsum`
//! count tables plus a deterministic ChaCha8 RNG. Fine for the corpus
//! sizes a single discussion sees (hundreds to low-thousands of
//! comments).

use crate::features::spaces::pages::apps::apps::analyzes::types::TopicRow;
// Pull `RngCore` + `SeedableRng` through `rand_chacha`'s re-exported
// `rand_core` to guarantee the traits match the `rand_core` version
// `ChaCha8Rng` implements. Pulling either from the top-level `rand`
// crate sometimes resolves to a different `rand_core` minor when the
// dep graph has multiple versions in flight (the workspace ends up
// with rand_core 0.5/0.6/0.9 simultaneously), and the type-checker
// then sees the trait as "different" from the one ChaCha8Rng
// implements — triggering "no function named seed_from_u64" or
// "no method named next_u32" errors despite the trait being in scope.
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::{RngCore, SeedableRng};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct LdaConfigV1 {
    /// Number of topics to extract.
    pub num_topics: usize,
    /// Gibbs iterations. 500 is the project default.
    pub iterations: usize,
    /// Top-N keywords per topic in the output.
    pub top_n: usize,
    /// Deterministic seed so successive runs over the same corpus
    /// produce identical topics — important so the user can tweak
    /// params and see a stable diff.
    pub seed: u64,
    /// Drop words appearing in fewer than `no_below` documents.
    pub no_below: usize,
    /// Drop words appearing in more than `no_above` fraction of
    /// documents (filters out generic/cross-topic terms).
    pub no_above: f64,
    /// Documents shorter than this after filtering are dropped.
    pub min_tokens_per_doc: usize,
    /// Dirichlet doc-topic prior.
    pub alpha: f64,
    /// Dirichlet topic-word prior.
    pub beta: f64,
}

impl Default for LdaConfigV1 {
    fn default() -> Self {
        Self {
            num_topics: 5,
            iterations: 500,
            top_n: 10,
            seed: 42,
            no_below: 2,
            no_above: 0.5,
            min_tokens_per_doc: 3,
            alpha: 0.1,
            beta: 0.01,
        }
    }
}

/// Strip terms that occur too rarely or too commonly across the
/// corpus, then drop documents that ended up too short. Mirrors the
/// gensim `filter_extremes` semantics.
fn filter_extremes(
    docs: Vec<Vec<String>>,
    no_below: usize,
    no_above: f64,
    min_tokens_per_doc: usize,
) -> Vec<Vec<String>> {
    let d = docs.len();
    if d == 0 {
        return vec![];
    }

    let mut df: HashMap<&str, usize> = HashMap::new();
    let ds = docs.clone();
    for doc in &ds {
        let mut seen = HashSet::new();
        for w in doc {
            if seen.insert(w.as_str()) {
                *df.entry(w.as_str()).or_insert(0) += 1;
            }
        }
    }

    let max_df = (no_above * d as f64).floor() as usize;

    docs.into_iter()
        .map(|doc| {
            doc.into_iter()
                .filter(|w| {
                    let c = df.get(w.as_str()).copied().unwrap_or(0);
                    c >= no_below && c <= max_df
                })
                .collect::<Vec<_>>()
        })
        .filter(|doc| doc.len() >= min_tokens_per_doc)
        .collect()
}

fn lda_from_tokens(docs: Vec<Vec<String>>, cfg: &LdaConfigV1) -> Vec<TopicRow> {
    let docs = filter_extremes(docs, cfg.no_below, cfg.no_above, cfg.min_tokens_per_doc);
    if docs.is_empty() {
        return vec![];
    }

    // Build word↔id index.
    let mut word2id: HashMap<String, usize> = HashMap::new();
    let mut id2word: Vec<String> = Vec::new();
    let mut docs_ids: Vec<Vec<usize>> = Vec::with_capacity(docs.len());

    for doc in docs {
        let mut v = Vec::with_capacity(doc.len());
        for w in doc {
            let id = *word2id.entry(w.clone()).or_insert_with(|| {
                let nid = id2word.len();
                id2word.push(w);
                nid
            });
            v.push(id);
        }
        docs_ids.push(v);
    }

    let d = docs_ids.len();
    let v = id2word.len();
    let k = cfg.num_topics;

    // Count tables — see Gibbs sampling for LDA paper.
    let mut nw = vec![vec![0usize; k]; v]; // nw[w][t] = count of word w assigned topic t
    let mut nd = vec![vec![0usize; k]; d]; // nd[doc][t] = count of tokens in doc with topic t
    let mut nwsum = vec![0usize; k];
    let mut ndsum = vec![0usize; d];
    let mut z: Vec<Vec<usize>> = Vec::with_capacity(d);

    let mut rng = ChaCha8Rng::seed_from_u64(cfg.seed);

    // Random initial assignment.
    for (di, doc) in docs_ids.iter().enumerate() {
        let mut zd = Vec::with_capacity(doc.len());
        ndsum[di] = doc.len();
        for &w in doc {
            let topic = (rng.next_u32() as usize) % k;
            zd.push(topic);
            nw[w][topic] += 1;
            nd[di][topic] += 1;
            nwsum[topic] += 1;
        }
        z.push(zd);
    }

    let alpha = cfg.alpha;
    let beta = cfg.beta;
    let vbeta = (v as f64) * beta;
    let mut p = vec![0f64; k];

    // Collapsed Gibbs sweep.
    for _ in 0..cfg.iterations {
        for di in 0..d {
            let doc = &docs_ids[di];
            for wi in 0..doc.len() {
                let w = doc[wi];
                let topic = z[di][wi];

                // Decrement current assignment.
                nw[w][topic] -= 1;
                nd[di][topic] -= 1;
                nwsum[topic] -= 1;

                // Compute conditional p(topic | rest).
                let mut total = 0.0;
                for t in 0..k {
                    let left = (nw[w][t] as f64 + beta) / (nwsum[t] as f64 + vbeta);
                    let right = (nd[di][t] as f64 + alpha)
                        / (ndsum[di] as f64 + (k as f64) * alpha);
                    p[t] = left * right;
                    total += p[t];
                }

                // Sample.
                let mut r = (rng.next_u32() as f64 / u32::MAX as f64) * total;
                let mut new_topic = 0usize;
                for t in 0..k {
                    r -= p[t];
                    if r <= 0.0 {
                        new_topic = t;
                        break;
                    }
                }

                z[di][wi] = new_topic;
                nw[w][new_topic] += 1;
                nd[di][new_topic] += 1;
                nwsum[new_topic] += 1;
            }
        }
    }

    // Build per-topic top-N keywords.
    let mut rows = Vec::with_capacity(k);
    for t in 0..k {
        let mut scores: Vec<(usize, f64)> = (0..v)
            .map(|wid| {
                let phi = (nw[wid][t] as f64 + beta) / (nwsum[t] as f64 + vbeta);
                (wid, phi)
            })
            .collect();
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let keywords: Vec<String> = scores
            .into_iter()
            .take(cfg.top_n)
            .map(|(wid, _)| id2word[wid].clone())
            .collect();

        rows.push(TopicRow {
            topic: format!("토픽_{}", t + 1),
            keywords,
        });
    }

    rows
}

/// Run LDA over a corpus of preprocessed token-vectors. Each entry in
/// `token_docs` is one document (the tokens of one comment). Returns
/// the top-N keywords per topic as `TopicRow`s.
pub fn run_lda(token_docs: Vec<Vec<String>>, cfg: LdaConfigV1) -> Vec<TopicRow> {
    let docs = token_docs
        .into_iter()
        .filter(|t| t.len() >= cfg.min_tokens_per_doc)
        .collect::<Vec<_>>();
    lda_from_tokens(docs, &cfg)
}
