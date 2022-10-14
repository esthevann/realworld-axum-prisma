pub fn check_if_following(follows: &Option<Vec<&str>>, user_id: &str) -> bool {
    if let Some(follows) = &follows {
        follows.iter().any(|x| *x == user_id)
    } else {
        false
    }
}

pub fn check_if_favorited(favorites: &Option<Vec<&str>>, article_id: &str) -> bool {
    if let Some(favorites) = &favorites {
        favorites.iter().any(|x| *x == article_id)
    } else {
        false
    }
}