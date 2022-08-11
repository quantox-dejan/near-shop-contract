use near_sdk::serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct ProductDto {
    id: String,
    name: String,
    price: f64
}

impl ProductDto {
    pub fn new(id: String, name: String, price: f64) -> Self {
        Self {
            id,
            name,
            price
        }
    }
}