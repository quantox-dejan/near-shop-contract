use crate::dto::product::ProductDto;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

#[derive(Default, Deserialize, Serialize)]
pub struct CouponDto {
    pub id: String,
    pub code: String,
    pub discount_percentage: f32,
    pub applies_to_all_products: bool,
    pub applies_to_products: Vec<ProductDto>,
    pub applies_to_all_users: bool,
    pub applies_to_user: Option<AccountId>,
    pub is_one_time: bool,
    pub times_used: u32,
}

impl CouponDto {
    pub fn new(
        id: String,
        code: String,
        discount_percentage: f32,
        applies_to_all_products: bool,
        applies_to_products: Vec<ProductDto>,
        applies_to_all_users: bool,
        applies_to_user: Option<AccountId>,
        is_one_time: bool,
        times_used: u32,
    ) -> Self {
        Self {
            id,
            code,
            discount_percentage,
            applies_to_all_products,
            applies_to_products,
            applies_to_all_users,
            applies_to_user,
            is_one_time,
            times_used,
        }
    }
}
