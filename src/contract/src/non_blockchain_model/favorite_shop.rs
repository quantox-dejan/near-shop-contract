use chrono::NaiveDateTime;
use diesel::Queryable;

use crate::utils::random_utils;

#[derive(Queryable)]
pub struct FavoriteShop {
    pub id: String,
    pub userid: String,
    pub shop: String,
    pub timestamp: NaiveDateTime,
}

impl FavoriteShop {
    pub fn new(user_id: &String, shop_id: &String) -> Self {
        Self {
            id: random_utils::get_random(),
            userid: String::from(user_id),
            shop: String::from(shop_id),
            timestamp: chrono::offset::Utc::now().naive_utc(),
        }
    }
}
