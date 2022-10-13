use crate::prisma::user;

pub fn check_if_following(user_logged: &user::Data, user_to_check: &user::Data) -> bool {
    if let Some(follows) = &user_logged.follows {
        follows.iter().any(|x| x.id == user_to_check.id)
    } else {
        false
    }
}

pub fn check_if_favorited(user_logged: &user::Data, article_id: &str) -> bool {
    if let Some(favorites) = &user_logged.favorites {
        favorites.iter().any(|x| x.id == article_id)
    } else {
        false
    }
}