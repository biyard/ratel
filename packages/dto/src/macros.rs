#[macro_export]
macro_rules! impl_display {
    ($type:ty) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let query = serde_urlencoded::to_string(&self).unwrap();
                write!(f, "{query}")
            }
        }
    };
}