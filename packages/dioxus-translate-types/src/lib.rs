pub trait Translator {
    fn en() -> Self;
    #[cfg(feature = "ko")]
    fn ko() -> Self;
}
