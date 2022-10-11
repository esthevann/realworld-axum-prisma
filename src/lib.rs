use axum::Router;

pub trait MergeRouter
where Self: Sized {
    fn merge_router<F: Fn(Self) -> Self>(self, f: F) -> Self;
}

impl<T> MergeRouter for Router<T>  {
    fn merge_router<F: Fn(Self) -> Self>(self, f: F) -> Self {
        (f)(self)
    }
}