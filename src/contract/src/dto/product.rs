use near_sdk::serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct ProductDto {
    pub id: String,
    pub name: String,
    pub price: u128,
    pub quantity_on_stock: i32,
}

impl ProductDto {
    pub fn new(id: String, name: String, price: u128, quantity_on_stock: i32) -> Self {
        Self {
            id,
            name,
            price,
            quantity_on_stock,
        }
    }
}
