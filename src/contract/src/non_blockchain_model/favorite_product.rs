use chrono::NaiveDateTime;
use diesel::Queryable;

use crate::utils::random_utils;

#[derive(Queryable)]
pub struct FavoriteProduct {
    pub id: String,
    pub userid: String,
    pub product: String,
    pub timestamp: NaiveDateTime,
}

impl FavoriteProduct {
    pub fn new(user_id: &String, product_id: &String) -> Self {
        Self {
            id: random_utils::get_random(),
            userid: String::from(user_id),
            product: String::from(product_id),
            timestamp: chrono::offset::Utc::now().naive_utc(),
        }
    }
}
