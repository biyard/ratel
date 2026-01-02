use crate::*;
use calamine::XlsxError;
use calamine::{Data, DataType as _, Reader, Xlsx, open_workbook};
use lindera::dictionary::load_dictionary;
use lindera::mode::Mode;
use lindera::segmenter::Segmenter;
use lindera::tokenizer::Tokenizer;
use once_cell::sync::OnceCell;
use rand::RngCore;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(
    Debug, Clone, Default, Serialize, Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct TopicRow {
    pub topic: String,
    pub keyword: String,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct LdaConfigV1 {
    pub num_topics: usize,
    pub iterations: usize,
    pub top_n: usize,
    pub seed: u64,
    pub no_below: usize,
    pub no_above: f64,
    pub min_tokens_per_doc: usize,
    pub alpha: f64,
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

fn tokenizer() -> &'static Tokenizer {
    static TOK: OnceCell<Tokenizer> = OnceCell::new();
    TOK.get_or_init(|| {
        let dictionary = load_dictionary("embedded://ko-dic").expect("load ko-dic");
        let segmenter = Segmenter::new(Mode::Normal, dictionary, None);
        Tokenizer::new(segmenter)
    })
}

fn is_content_noun(pos: &str) -> bool {
    matches!(pos, "NNG" | "NNP" | "NR")
}

fn stopwords_set() -> HashSet<&'static str> {
    [
        "것",
        "수",
        "등",
        "때",
        "곳",
        "내",
        "중",
        "년",
        "명",
        "개",
        "점",
        "번",
        "차",
        "경우",
        "정도",
        "말",
        "거",
        "게",
        "데",
        "분",
        "부분",
        "전",
        "후",
        "측",
        "쪽",
        "그것",
        "이것",
        "생각",
        "때문",
        "동의",
        "비동",
        "비동의",
    ]
    .into_iter()
    .collect()
}

fn token_pos_surface(token: &mut lindera::token::Token) -> (String, String) {
    let surface = token.surface.to_string();
    let pos = token
        .details()
        .get(0)
        .map(|v| (*v).to_string())
        .unwrap_or_default();
    (surface, pos)
}

fn has_jongseong(syllable: char) -> Option<bool> {
    let u = syllable as u32;
    if (0xAC00..=0xD7A3).contains(&u) {
        Some(((u - 0xAC00) % 28) != 0)
    } else {
        None
    }
}

fn strip_attached_particle(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    if chars.len() < 2 {
        return word.to_string();
    }
    let last = *chars.last().unwrap();
    let prev = chars[chars.len() - 2];

    let prev_has_batchim = match has_jongseong(prev) {
        Some(v) => v,
        None => return word.to_string(),
    };

    let should_strip = match last {
        '이' | '을' | '은' | '과' => prev_has_batchim,
        '가' | '를' | '는' | '와' => !prev_has_batchim,
        _ => false,
    };

    if should_strip {
        chars.pop();
        return chars.into_iter().collect();
    }

    word.to_string()
}

pub fn preprocess_korean_nouns(text: &str) -> Vec<String> {
    let tok = tokenizer();
    let mut tokens = match tok.tokenize(text) {
        Ok(t) => t,
        Err(_) => return vec![],
    };

    let stopwords = stopwords_set();
    let mut stream: Vec<(String, String)> = Vec::with_capacity(tokens.len());

    for token in tokens.iter_mut() {
        let (surface, pos) = token_pos_surface(token);
        stream.push((surface, pos));
    }

    let mut merged: Vec<(String, String)> = Vec::with_capacity(stream.len());
    let mut i = 0usize;
    while i < stream.len() {
        let (w1, p1) = &stream[i];

        if i + 1 < stream.len() {
            let (w2, _p2) = &stream[i + 1];

            let forced = match (w1.as_str(), w2.as_str()) {
                ("성관", "계") => Some("성관계"),
                ("간음", "죄") => Some("간음죄"),
                ("무고", "죄") => Some("무고죄"),
                _ => None,
            };

            if let Some(joined) = forced {
                merged.push((joined.to_string(), p1.clone()));
                i += 2;
                continue;
            }
        }

        merged.push((w1.clone(), p1.clone()));
        i += 1;
    }

    let mut out = Vec::new();

    for i in 0..merged.len() {
        let (word0, pos) = &merged[i];

        if !is_content_noun(pos) {
            continue;
        }

        let word = strip_attached_particle(word0);

        if word.chars().count() < 2 {
            continue;
        }
        if !word.chars().all(|c| ('가'..='힣').contains(&c)) {
            continue;
        }

        if word == "비동의" {
            continue;
        }
        if word == "비동" && i + 1 < merged.len() && merged[i + 1].0 == "의" {
            continue;
        }
        if word == "동의" && i > 0 && merged[i - 1].0 == "비" {
            continue;
        }

        if stopwords.contains(word.as_str()) {
            continue;
        }

        if word == "간음" && i + 1 < merged.len() && merged[i + 1].0 == "죄" {
            continue;
        }

        out.push(word);
    }

    out
}

fn cell_string(v: &Data) -> Option<String> {
    if let Some(s) = v.get_string() {
        let t = s.trim();
        return if t.is_empty() {
            None
        } else {
            Some(t.to_string())
        };
    }
    if let Some(f) = v.get_float() {
        if !f.is_nan() {
            return Some(format!("{}", f));
        }
    }
    if let Some(i) = v.get_int() {
        return Some(format!("{}", i));
    }
    if let Some(b) = v.get_bool() {
        return Some(format!("{}", b));
    }
    None
}

fn gender_label(v: &Data) -> Option<&'static str> {
    let n = if let Some(i) = v.get_int() {
        i
    } else if let Some(f) = v.get_float() {
        f as i64
    } else if let Some(s) = v.get_string() {
        s.trim().parse::<i64>().ok()?
    } else {
        return None;
    };

    match n {
        1 => Some("남성"),
        2 => Some("여성"),
        _ => None,
    }
}

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

    let mut nw = vec![vec![0usize; k]; v];
    let mut nd = vec![vec![0usize; k]; d];
    let mut nwsum = vec![0usize; k];
    let mut ndsum = vec![0usize; d];
    let mut z: Vec<Vec<usize>> = Vec::with_capacity(d);

    let mut rng = ChaCha8Rng::seed_from_u64(cfg.seed);

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

    for _ in 0..cfg.iterations {
        for di in 0..d {
            let doc = &docs_ids[di];
            for wi in 0..doc.len() {
                let w = doc[wi];
                let topic = z[di][wi];

                nw[w][topic] -= 1;
                nd[di][topic] -= 1;
                nwsum[topic] -= 1;

                let mut total = 0.0;
                for t in 0..k {
                    let left = (nw[w][t] as f64 + beta) / (nwsum[t] as f64 + vbeta);
                    let right =
                        (nd[di][t] as f64 + alpha) / (ndsum[di] as f64 + (k as f64) * alpha);
                    p[t] = left * right;
                    total += p[t];
                }

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

    let mut rows = Vec::new();

    for t in 0..k {
        let mut scores: Vec<(usize, f64)> = (0..v)
            .map(|wid| {
                let phi = (nw[wid][t] as f64 + beta) / (nwsum[t] as f64 + vbeta);
                (wid, phi)
            })
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        for (wid, phi) in scores.into_iter().take(cfg.top_n) {
            rows.push(TopicRow {
                topic: format!("토픽_{}", t + 1),
                keyword: id2word[wid].clone(),
                weight: phi,
            });
        }
    }

    rows
}

pub fn run_from_xlsx(
    path: &str,
    cfg: LdaConfigV1,
) -> crate::Result<HashMap<String, Vec<TopicRow>>> {
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

    let mut comments_data: HashMap<String, Vec<String>> = HashMap::from([
        ("1차_남성".to_string(), vec![]),
        ("1차_여성".to_string(), vec![]),
        ("2차_통합".to_string(), vec![]),
    ]);

    let all_rows: Vec<Vec<Data>> = rows_iter.map(|r| r.to_vec()).collect();

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

    let mut out: HashMap<String, Vec<TopicRow>> = HashMap::new();

    for (group, comments) in comments_data {
        let token_docs = comments
            .iter()
            .map(|c| preprocess_korean_nouns(c))
            .filter(|t| t.len() >= cfg.min_tokens_per_doc)
            .collect::<Vec<_>>();

        let rows = lda_from_tokens(token_docs, &cfg);
        out.insert(group, rows);
    }

    Ok(out)
}
