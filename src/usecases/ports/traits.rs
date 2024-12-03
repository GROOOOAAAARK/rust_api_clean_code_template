pub trait Opt<T> {
    fn unwrap_or(self, default: T) -> T;
}
