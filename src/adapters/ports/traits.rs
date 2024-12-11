use crate::usecases::ports::response::Response;

pub trait ResponseConverter<T> {
    fn from_internal_response(&self, response: Response) -> T;
}

pub trait PresenterTrait<T> {
    fn present(&self, response: Response) -> T;
}

pub trait AdapterTrait<T, R> {
    fn invoke(&self, input: T) -> R;
}