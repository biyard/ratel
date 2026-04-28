//! TF-IDF computation across the matched-user comment corpus.
//!
//! Each comment is one document. For every term, we sum its
//! TF-IDF score across documents (`tf * idf`) and surface the top-N.
//! `relative` is the score normalised against the max so the bar
//! widths in the UI are direct percentages.

use crate::features::spaces::pages::apps::apps::analyzes::types::TermScore;
use std::collections::HashMap;

pub fn run_tfidf(token_docs: &[Vec<String>], top_n: usize) -> Vec<TermScore> {
    let n_docs = token_docs.len();
    if n_docs == 0 || top_n == 0 {
        return Vec::new();
    }

    // Term frequency per (term, doc) and document frequency per term.
    let mut tf_per_term: HashMap<String, Vec<f64>> = HashMap::new();
    let mut df: HashMap<String, usize> = HashMap::new();

    for (di, doc) in token_docs.iter().enumerate() {
        if doc.is_empty() {
            continue;
        }
        let mut local: HashMap<&str, f64> = HashMap::new();
        for tok in doc {
            *local.entry(tok.as_str()).or_insert(0.0) += 1.0;
        }
        let len = doc.len() as f64;
        for (tok, count) in local {
            let entry = tf_per_term
                .entry(tok.to_string())
                .or_insert_with(|| vec![0.0; n_docs]);
            entry[di] = count / len;
            *df.entry(tok.to_string()).or_insert(0) += 1;
        }
    }

    let n_docs_f = n_docs as f64;
    let mut scores: Vec<(String, f64)> = tf_per_term
        .iter()
        .map(|(term, tfs)| {
            let df_t = *df.get(term).unwrap_or(&1) as f64;
            // Smoothed IDF — `1 + ln(N/df)` keeps terms appearing in
            // every doc from collapsing to 0.
            let idf = 1.0 + (n_docs_f / df_t).ln();
            let score: f64 = tfs.iter().map(|tf| tf * idf).sum();
            (term.clone(), score)
        })
        .collect();

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scores.truncate(top_n);

    let max = scores
        .first()
        .map(|(_, s)| *s)
        .filter(|s| *s > 0.0)
        .unwrap_or(1.0);

    scores
        .into_iter()
        .map(|(term, score)| TermScore {
            term,
            score,
            relative: (score / max).clamp(0.0, 1.0),
        })
        .collect()
}
