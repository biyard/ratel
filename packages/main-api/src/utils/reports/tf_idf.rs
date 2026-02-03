use crate::utils::reports::preprocess_korean_nouns;
use crate::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TfidfConfigV1 {
    // 최대 키워드 수
    pub max_features: usize,
    // 문서 내 단어 등장 횟수
    pub min_df: usize,
    // 단어 최소 크기
    pub ngram_min: usize,
    // 단어 최대 크기
    pub ngram_max: usize,
    // 문장 내 최소 토큰 수
    pub min_tokens_per_doc: usize,
}

impl Default for TfidfConfigV1 {
    fn default() -> Self {
        Self {
            max_features: 100,
            ngram_min: 1,
            ngram_max: 2,
            min_df: 2,
            min_tokens_per_doc: 1,
        }
    }
}

fn make_terms(tokens: &[String], n_min: usize, n_max: usize) -> Vec<String> {
    if tokens.is_empty() {
        return vec![];
    }

    let mut out = Vec::new();

    if n_min <= 1 && n_max >= 1 {
        out.extend(tokens.iter().cloned());
    }

    if n_min <= 2 && n_max >= 2 && tokens.len() >= 2 {
        for i in 0..tokens.len() - 1 {
            out.push(format!("{} {}", tokens[i], tokens[i + 1]));
        }
    }

    out
}

fn smooth_idf(n_docs: usize, df: usize) -> f64 {
    (((1.0 + n_docs as f64) / (1.0 + df as f64)).ln()) + 1.0
}

fn l2_norm(v: &[f64]) -> f64 {
    let mut s = 0.0;
    for x in v {
        s += x * x;
    }
    s.sqrt()
}

fn compute_tfidf(docs_tokens: Vec<Vec<String>>, cfg: &TfidfConfigV1) -> Vec<TfidfRow> {
    let mut docs_tokens = docs_tokens
        .into_iter()
        .filter(|t| t.len() >= cfg.min_tokens_per_doc)
        .collect::<Vec<_>>();

    if docs_tokens.is_empty() {
        return vec![];
    }

    for doc in docs_tokens.iter_mut() {
        doc.retain(|w| w != "성이");
    }
    docs_tokens.retain(|d| !d.is_empty());
    if docs_tokens.is_empty() {
        return vec![];
    }

    let mut docs_tf: Vec<HashMap<String, usize>> = Vec::with_capacity(docs_tokens.len());
    for tokens in &docs_tokens {
        let terms = make_terms(tokens, cfg.ngram_min, cfg.ngram_max);
        if terms.is_empty() {
            continue;
        }
        let mut tf = HashMap::<String, usize>::new();
        for t in terms {
            *tf.entry(t).or_insert(0) += 1;
        }
        docs_tf.push(tf);
    }

    let n_docs = docs_tf.len();
    if n_docs == 0 {
        return vec![];
    }

    let mut df: HashMap<String, usize> = HashMap::new();
    for tf in &docs_tf {
        for term in tf.keys() {
            *df.entry(term.clone()).or_insert(0) += 1;
        }
    }

    let mut total_tf: HashMap<String, usize> = HashMap::new();
    for tf in &docs_tf {
        for (term, c) in tf {
            let dfi = df.get(term).copied().unwrap_or(0);
            if dfi >= cfg.min_df {
                *total_tf.entry(term.clone()).or_insert(0) += *c;
            }
        }
    }

    let mut feats: Vec<(String, usize)> = total_tf.into_iter().collect();
    feats.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    feats.truncate(cfg.max_features);

    if feats.is_empty() {
        return vec![];
    }

    let vocab: Vec<String> = feats.into_iter().map(|(t, _)| t).collect();
    let m = vocab.len();

    let mut idf = vec![0.0f64; m];
    for (j, term) in vocab.iter().enumerate() {
        let dfi = df.get(term).copied().unwrap_or(0);
        idf[j] = smooth_idf(n_docs, dfi);
    }

    let mut col_sum = vec![0.0f64; m];

    for tf in &docs_tf {
        let mut w = vec![0.0f64; m];
        for j in 0..m {
            let term = &vocab[j];
            let f = tf.get(term).copied().unwrap_or(0) as f64;
            w[j] = f * idf[j];
        }

        let norm = l2_norm(&w);
        if norm > 0.0 {
            for j in 0..m {
                col_sum[j] += w[j] / norm;
            }
        }
    }

    let mut rows = Vec::with_capacity(m);
    for j in 0..m {
        rows.push(TfidfRow {
            keyword: vocab[j].clone(),
            tf_idf: col_sum[j],
        });
    }

    rows.sort_by(|a, b| b.tf_idf.partial_cmp(&a.tf_idf).unwrap());
    rows
}

// TODO: check tf-idf logic
pub fn run_tfidf(
    comments: &[String],
    cfg: TfidfConfigV1,
    remove_topics: &[String],
) -> crate::Result<Vec<TfidfRow>> {
    let docs_tokens = comments
        .iter()
        .map(|c| preprocess_korean_nouns(c, remove_topics))
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>();

    Ok(compute_tfidf(docs_tokens, &cfg))
}
