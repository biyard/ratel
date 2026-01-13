use lindera::dictionary::load_dictionary;
use lindera::mode::Mode;
use lindera::segmenter::Segmenter;
use lindera::tokenizer::Tokenizer;
use once_cell::sync::OnceCell;
use std::collections::{HashMap, HashSet};

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

pub fn preprocess_korean_nouns(text: &str) -> Vec<String> {
    let tok = tokenizer();
    let mut tokens = match tok.tokenize(text) {
        Ok(t) => t,
        Err(_) => return vec![],
    };

    let stopwords = stopwords_set();
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

        if word == "성이" && i + 1 < stream.len() && stream[i + 1].0 == "해" {
            out.push("성이해".to_string());
            i += 2;
            continue;
        }

        if word == "준이" && i > 0 && stream[i - 1].0 == "기" {
            out.push("기준".to_string());
            i += 1;
            continue;
        }

        if word == "성관" && i + 1 < stream.len() && stream[i + 1].0 == "계" {
            out.push("성관계".to_string());
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

        if word == "비동의" {
            i += 1;
            continue;
        }
        if word == "비동" && i + 1 < stream.len() && stream[i + 1].0 == "의" {
            i += 1;
            continue;
        }
        if word == "동의" && i > 0 && stream[i - 1].0 == "비" {
            i += 1;
            continue;
        }

        if stopwords.contains(word.as_str()) {
            i += 1;
            continue;
        }

        if word == "간음" && i + 1 < stream.len() && stream[i + 1].0 == "죄" {
            i += 1;
            continue;
        }

        out.push(word.clone());
        i += 1;
    }

    out
}
