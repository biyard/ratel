pub trait Bookmarker<T> {
    fn bookmark(&self) -> Option<T>;
}
