use lindera::dictionary::load_dictionary;
use lindera::mode::Mode;
use lindera::segmenter::Segmenter;
use lindera::tokenizer::Tokenizer;
use once_cell::sync::OnceCell;
use std::collections::HashSet;

fn tokenizer() -> &'static Tokenizer {
    static TOK: OnceCell<Tokenizer> = OnceCell::new();
    TOK.get_or_init(|| {
        let dictionary = load_dictionary("embedded://ko-dic").expect("load ko-dic");
        let segmenter = Segmenter::new(Mode::Normal, dictionary, None);
        Tokenizer::new(segmenter)
    })
}

fn is_noun_pos(pos: &str) -> bool {
    pos.starts_with("NN") || pos == "NR" || pos == "NP"
}

fn stopwords_set() -> HashSet<&'static str> {
    [
        "것", "수", "등", "때", "곳", "내", "중", "년", "명", "개", "점", "번", "차", "경우",
        "정도", "말", "거", "게", "데", "분", "부분", "전", "후", "측", "쪽", "그것", "이것",
        "생각", "때문",
    ]
    .into_iter()
    .collect()
}

pub fn preprocess_korean_nouns(text: &str, remove_topics: &[String]) -> Vec<String> {
    let tok = tokenizer();
    let mut tokens = match tok.tokenize(text) {
        Ok(t) => t,
        Err(_) => return vec![],
    };

    let stopwords = stopwords_set();
    let remove_set: HashSet<&str> = remove_topics.iter().map(|v| v.as_str()).collect();
    let mut stream: Vec<(String, String)> = Vec::with_capacity(tokens.len());

    for token in tokens.iter_mut() {
        let surface = token.surface.to_string();
        let pos = token
            .details()
            .get(0)
            .map(|v| (*v).to_string())
            .unwrap_or_default();
        stream.push((surface, pos));
    }

    let mut out = Vec::new();
    let mut i = 0usize;

    while i < stream.len() {
        let (word, pos) = &stream[i];

        if i + 1 < stream.len() {
            let joined = format!("{}{}", word, stream[i + 1].0);
            if remove_set.contains(joined.as_str()) {
                i += 2;
                continue;
            }
        }

        if remove_set.contains(word.as_str()) {
            i += 1;
            continue;
        }

        if word == "성이" && i + 1 < stream.len() && stream[i + 1].0 == "해" {
            let merged = "성이해".to_string();
            if !remove_set.contains(merged.as_str()) {
                out.push(merged);
            }
            i += 2;
            continue;
        }

        if word == "준이" && i > 0 && stream[i - 1].0 == "기" {
            let merged = "기준".to_string();
            if !remove_set.contains(merged.as_str()) {
                out.push(merged);
            }
            i += 1;
            continue;
        }

        if word == "성관" && i + 1 < stream.len() && stream[i + 1].0 == "계" {
            let merged = "성관계".to_string();
            if !remove_set.contains(merged.as_str()) {
                out.push(merged);
            }
            i += 2;
            continue;
        }

        if !is_noun_pos(pos) {
            i += 1;
            continue;
        }
        if word.chars().count() < 2 {
            i += 1;
            continue;
        }
        if !word.chars().all(|c| ('가'..='힣').contains(&c)) {
            i += 1;
            continue;
        }

        if stopwords.contains(word.as_str()) {
            i += 1;
            continue;
        }

        out.push(word.clone());
        i += 1;
    }

    out
}
