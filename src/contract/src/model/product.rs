use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

use crate::utils::random_utils;

#[derive(Default, BorshDeserialize, BorshSerialize, Clone, PartialEq, Debug)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: u128,
    pub quantity_on_stock: i32,
}

impl Product {
    pub fn new(name: String, price: u128, quantity_on_stock: i32) -> Self {
        Self {
            id: random_utils::get_random(),
            name,
            price,
            quantity_on_stock,
        }
    }
}
