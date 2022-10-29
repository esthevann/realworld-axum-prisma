use axum::Router;

pub trait MergeRouter
where
    Self: Sized,
{
    fn merge_router<F: Fn(Self) -> Self>(self, f: F) -> Self;
}

impl<T> MergeRouter for Router<T> {
    fn merge_router<F: Fn(Self) -> Self>(self, f: F) -> Self {
        (f)(self)
    }
}

pub fn check_if_following<T: AsRef<str>>(follows: &[T], user_id: &str) -> bool {
    follows.iter().any(|x| x.as_ref() == user_id)
}

pub fn check_if_favorited<T: AsRef<str>>(favorites: &[T], article_id: &str) -> bool {
    favorites.iter().any(|x| x.as_ref() == article_id)
}
