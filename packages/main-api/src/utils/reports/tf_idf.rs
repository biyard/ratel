use crate::utils::reports::{cell_string, gender_label, preprocess_korean_nouns};
use crate::*;
use calamine::XlsxError;
use calamine::{Data, Reader, Xlsx, open_workbook};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(
    Debug, Clone, Default, Serialize, Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct TfidfRow {
    pub keyword: String,
    pub tf_idf: f64,
}

#[derive(Debug, Clone)]
pub struct TfidfConfigV1 {
    pub max_features: usize,
    pub min_df: usize,
    pub ngram_min: usize,
    pub ngram_max: usize,
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

pub fn run_tfidf_from_xlsx(
    path: &str,
    cfg: TfidfConfigV1,
) -> crate::Result<HashMap<String, Vec<TfidfRow>>> {
    let mut workbook: Xlsx<_> = open_workbook(path)
        .map_err(|e: XlsxError| crate::Error::InternalServerError(e.to_string()))?;

    let range = workbook
        .worksheet_range("raw")
        .map_err(|e: XlsxError| crate::Error::InternalServerError(e.to_string()))?;

    let mut rows_iter = range.rows();
    let header_row = rows_iter
        .next()
        .ok_or_else(|| crate::Error::InternalServerError("empty sheet".to_string()))?;

    let headers: Vec<String> = header_row.iter().map(|c| c.to_string()).collect();

    let gender_idx = headers
        .iter()
        .position(|h| h.trim() == "Gender")
        .ok_or_else(|| crate::Error::InternalServerError("missing column: Gender".to_string()))?;

    let mut cols_1st = Vec::new();
    let mut cols_2nd = Vec::new();

    for (i, h) in headers.iter().enumerate() {
        let hl = h.to_lowercase();
        if h.contains("1차") && !hl.contains("type") {
            cols_1st.push(i);
        }
        if h.contains("2차") && !hl.contains("type") {
            cols_2nd.push(i);
        }
    }

    let all_rows: Vec<Vec<Data>> = rows_iter.map(|r| r.to_vec()).collect();

    let mut comments_data: HashMap<String, Vec<String>> = HashMap::from([
        ("1차_남성".to_string(), vec![]),
        ("1차_여성".to_string(), vec![]),
        ("2차_통합".to_string(), vec![]),
    ]);

    for r in &all_rows {
        let gender = r.get(gender_idx).and_then(gender_label);
        if let Some(g) = gender {
            let key = format!("1차_{}", g);
            if let Some(v) = comments_data.get_mut(&key) {
                for &ci in &cols_1st {
                    if let Some(cell) = r.get(ci) {
                        if let Some(s) = cell_string(cell) {
                            v.push(s);
                        }
                    }
                }
            }
        }
    }

    for r in &all_rows {
        if let Some(v) = comments_data.get_mut("2차_통합") {
            for &ci in &cols_2nd {
                if let Some(cell) = r.get(ci) {
                    if let Some(s) = cell_string(cell) {
                        v.push(s);
                    }
                }
            }
        }
    }

    let mut out: HashMap<String, Vec<TfidfRow>> = HashMap::new();

    for (group, comments) in comments_data {
        let docs_tokens = comments
            .iter()
            .map(|c| preprocess_korean_nouns(c))
            .filter(|t| !t.is_empty())
            .collect::<Vec<_>>();

        let rows = compute_tfidf(docs_tokens, &cfg);
        out.insert(group, rows);
    }

    Ok(out)
}
