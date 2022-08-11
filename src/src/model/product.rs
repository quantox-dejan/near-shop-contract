use nanoid::nanoid;
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};

#[derive(Default, BorshDeserialize, BorshSerialize, Clone, PartialEq)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64
}

impl Product {
    pub fn new(name: String, price: f64) -> Self {
        Self {
            id: nanoid!(),
            name,
            price
        }
    }
}