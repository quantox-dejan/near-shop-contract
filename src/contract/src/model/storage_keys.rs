use near_sdk::borsh::{self, BorshSerialize};
use near_sdk::BorshStorageKey;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    UserShops,
    Products  { user_shop: Vec<u8> },
    Coupons { user_shop: Vec<u8> }
}