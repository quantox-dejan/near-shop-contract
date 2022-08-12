use nanoid::nanoid;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, PanicOnDefault};

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Debug, Clone)]
pub struct Coupon {
    pub id: String,
    pub code: String,
    pub discount_percentage: f32,
    pub applies_to_all_products: bool,
    pub applies_to_products: Vec<String>,
    pub applies_to_all_users: bool,
    pub applies_to_user: Option<AccountId>,
    pub is_one_time: bool,
    pub times_used: u32,
}

impl Coupon {
    pub fn new(code: String, discount_percentage: f32) -> Self {
        Self {
            id: nanoid!(),
            code,
            discount_percentage,
            applies_to_all_products: true,
            applies_to_products: vec![],
            applies_to_all_users: true,
            applies_to_user: None,
            is_one_time: false,
            times_used: 0,
        }
    }

    pub fn specific_new(
        code: String,
        discount_percentage: f32,
        applies_to_products: &Vec<String>,
        applies_to_user: Option<AccountId>,
        is_one_time: bool,
    ) -> Self {
        Self {
            id: nanoid!(),
            code,
            discount_percentage,
            applies_to_all_products: applies_to_products.is_empty(),
            applies_to_products: applies_to_products.to_vec(),
            applies_to_all_users: applies_to_user.is_none(),
            applies_to_user,
            is_one_time,
            times_used: 0,
        }
    }
}
