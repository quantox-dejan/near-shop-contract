use near_sdk::serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserShopDto {
    pub id: String,
    pub name: String,
}

impl UserShopDto {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name
        }
    }
}