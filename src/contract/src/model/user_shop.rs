use super::coupon::Coupon;
use super::product::Product;
use super::storage_keys::StorageKeys;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{env, AccountId};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct UserShop {
    pub id: String,
    pub name: String,
    pub owner_account_id: AccountId,
    pub products: Vector<Product>,
    pub coupons: Vector<Coupon>,
}

impl UserShop {
    pub fn new(name: String) -> Self {
        let id = nanoid::nanoid!();
        Self {
            id: String::from(&id),
            name,
            owner_account_id: env::predecessor_account_id(),
            products: Vector::new(StorageKeys::Products {
                user_shop: env::sha256(id.as_bytes()),
            }),
            coupons: Vector::new(StorageKeys::Coupons {
                user_shop: env::sha256(id.as_bytes()),
            }),
        }
    }
}
