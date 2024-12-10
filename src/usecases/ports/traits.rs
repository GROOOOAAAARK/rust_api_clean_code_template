use crate::usecases::ports::response::Response;

pub trait Opt<T> {
    fn unwrap_or(self, default: T) -> T;
}

pub trait UsecaseTrait<T> {
    fn execute(&self, input: T) -> Response;
}