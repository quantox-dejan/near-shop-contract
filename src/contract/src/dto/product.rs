use near_sdk::{
    json_types::U128,
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct ProductDto {
    pub id: String,
    pub name: String,
    pub price: U128,
    pub quantity_on_stock: i32,
}

impl ProductDto {
    pub fn new(id: String, name: String, price: u128, quantity_on_stock: i32) -> Self {
        Self {
            id,
            name,
            price: U128(price),
            quantity_on_stock,
        }
    }
}
