//! Text-analysis primitives for the user-triggered discussion flow.
//!
//! - `preprocess`: ko-dic morphological filter → Korean noun tokens
//! - `lda`: collapsed Gibbs sampling LDA topic modelling
//! - `tfidf`: per-term TF-IDF score over the corpus
//! - `text_network`: word co-occurrence graph nodes + edges
//!
//! All four operate on the same canonical "tokens per document" shape
//! produced by `preprocess::preprocess_korean_nouns`. Lambda glue in
//! `services::discussion_analysis` chains them in order.

pub mod lda;
pub mod preprocess;
pub mod text_network;
pub mod tfidf;
