//! Korean noun preprocessing for the discussion analysis pipeline.
//!
//! Pipes raw comment text through `lindera` (ko-dic) and keeps only
//! the noun-tagged surface forms that pass the project's filter rules.
//! Output is what every downstream module — LDA, TF-IDF, text-network —
//! consumes as the canonical "tokens for this document".
//!
//! Dictionary loading is one-shot via `OnceCell`: the embedded ko-dic
//! costs ~80 MB to materialise, so amortising it across every comment
//! in a request is essential. The Lambda runtime is long-lived (warm
//! starts) so this also pays for itself across multiple invocations.

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

/// ko-dic noun-family POS tags. NN* covers regular/proper nouns,
/// NR is a numeral, NP is a pronoun. We treat them all as noun-like
/// for downstream analysis since topic / co-occurrence work fine on
/// pronouns and numerals (and stop-words handle the noisy ones).
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

/// Tokenise `text` into a vec of Korean noun surface forms ready for
/// analysis. Returns an empty vec on tokeniser failure rather than
/// erroring — text analysis should degrade gracefully when one
/// document is malformed.
///
/// `extra_stopwords` lets the caller layer additional words on top
/// of the project-wide list — used by the discussion form's
/// "제외된 토픽" input so user-supplied exclusions kick in without
/// recompiling the static set.
pub fn preprocess_korean_nouns(text: &str, extra_stopwords: &HashSet<String>) -> Vec<String> {
    let tok = tokenizer();
    let mut tokens = match tok.tokenize(text) {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };

    let stopwords = stopwords_set();

    // Collect (surface, pos) pairs first so we can reach back/forward
    // for the small set of bigram fixups we need (성이→성이해,
    // 기→준이→기준 etc.). Doing this in two passes keeps the rule
    // application straightforward.
    let mut stream: Vec<(String, String)> = Vec::with_capacity(tokens.len());
    for token in tokens.iter_mut() {
        let surface = token.surface.to_string();
        let pos = token
            .details()
            .first()
            .map(|v| (*v).to_string())
            .unwrap_or_default();
        stream.push((surface, pos));
    }

    let mut out = Vec::new();
    let mut i = 0usize;

    while i < stream.len() {
        let (word, pos) = &stream[i];

        // ── Bigram fixups ko-dic mis-segments for this corpus ──
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

        // ── Filter: noun-only, length ≥ 2, hangul-only ──
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

        // ── Drop stop tokens that produce false 비동의/동의 noise ──
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

        // Domain-specific: "간음" preceding "죄" gets dropped — the
        // legally meaningful noun is "강간죄" / "비동의간음죄" but
        // ko-dic splits them awkwardly.
        if word == "간음" && i + 1 < stream.len() && stream[i + 1].0 == "죄" {
            i += 1;
            continue;
        }

        // User-supplied exclusions (case-insensitive lower-cased).
        if extra_stopwords.contains(&word.to_lowercase()) {
            i += 1;
            continue;
        }

        out.push(word.clone());
        i += 1;
    }

    out
}
