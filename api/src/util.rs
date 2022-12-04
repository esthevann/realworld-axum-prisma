pub fn check_if_following<T: AsRef<str>>(follows: &[T], user_id: &str) -> bool {
    follows.iter().any(|x| x.as_ref() == user_id)
}

pub fn check_if_favorited<T: AsRef<str>>(favorites: &[T], article_id: &str) -> bool {
    favorites.iter().any(|x| x.as_ref() == article_id)
}
