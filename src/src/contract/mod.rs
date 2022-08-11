mod near_shop;

use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{UnorderedMap};
use crate::contract::near_shop::NearShopContract;
use crate::dto::coupon::CouponDto;
use crate::dto::product::ProductDto;
use crate::dto::user_shop::UserShopDto;
use crate::model::coupon::Coupon;
use crate::model::product::Product;
use crate::model::user_shop::UserShop;
use crate::model::storage_keys::StorageKeys;
use crate::utils::vector_utils::VectorUtils;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NearShop {
    /// The lazy map of user shops.
    /// We don't need to read the entire map of existing user shops when adding a new one.
    /// Each time the state is loaded, using the standard Map would cause all entries
    /// in the data structure to be read eagerly from storage and deserialized.
    /// This would come at a large cost for any non-trivial amount of data,
    /// so to minimize the amount of gas used the SDK UnorderedMap is used.
    user_shops: UnorderedMap<AccountId, UserShop>
}

#[near_bindgen]
impl NearShopContract for NearShop {
    #[init]
    fn new() -> Self {
        if !env::state_read::<Self>().is_none() {
            env::panic_str("Already initialized");
        }

        Self {
            user_shops: UnorderedMap::new(StorageKeys::UserShops)
        }
    }

    fn get_my_user_shop(&self) -> Option<UserShopDto> {
        let user_shop = self.user_shops.get(&env::predecessor_account_id());
        match user_shop {
            Some(result) => {
                Some(UserShopDto {
                    id: result.id,
                    name: result.name
                })
            }
            None => {
                None
            }
        }
    }

    fn list_all_user_shops(&self) -> Vec<UserShopDto> {
        let keys = self.user_shops.keys_as_vector();
        let mut return_value = Vec::new();
        for key in keys.iter() {
            let user_shop_maybe = self.user_shops.get(&key);
            if let Some(user_shop) = user_shop_maybe {
                return_value.push(UserShopDto::new(user_shop.id, user_shop.name));
            }
        }

        return_value
    }

    fn list_user_shop_products(&self, user_shop_id: String) -> Vec<ProductDto> {
        let user_shops = self.user_shops.values_as_vector().to_vec();
        let found_user_shop = user_shops.iter().find(|&x| x.id == user_shop_id);
        let mut return_value = Vec::new();
        if let Some(user_shop) = found_user_shop {
            for product in user_shop.products.to_vec().iter() {
                return_value.push(ProductDto::new(String::from(&product.id), String::from(&product.name), product.price))
            }
        }

        return_value
    }

    fn list_my_user_shop_coupons(&self) -> Vec<CouponDto> {
        let user_shop_maybe = self.user_shops.get(&env::predecessor_account_id());
        if let None = user_shop_maybe {
            env::panic_str("You don't have a shop");
        }

        let user_shop = user_shop_maybe.unwrap();
        let mut return_value = Vec::new();
        for coupon in user_shop.coupons.to_vec().iter() {
            let dto = CouponDto::new(
                String::from(&coupon.id),
                String::from(&coupon.code),
                coupon.discount_percentage,
                coupon.applies_to_all_products,
                self.convert_products(&user_shop, &coupon.applies_to_products),
                coupon.applies_to_all_users,
                coupon.applies_to_user.clone(),
                coupon.is_one_time,
                coupon.times_used
            );

            return_value.push(dto);
        }

        return_value
    }

    fn add_user_shop(&mut self, name: String) {
        let existing_user_shop = self.user_shops.get(&env::predecessor_account_id());
        if !existing_user_shop.is_none() {
            env::panic_str("You already have a shop");
        }

        self.user_shops.insert(&env::predecessor_account_id(), &UserShop::new(name));
    }

    fn add_product(&mut self, name: String, price: f64) {
        let user_shop = self.user_shops.get(&env::predecessor_account_id());
        match user_shop {
            Some(mut result) => {
                result.products.push(&Product::new(name, price));
            }
            None => {
                env::panic_str("You need to register your shop before adding products to sell");
            }
        }
    }

    fn add_default_coupon(&mut self, code: String, discount_percentage: f32) {
        let user_shop_maybe = self.user_shops.get(&env::predecessor_account_id());
        match user_shop_maybe {
            Some(mut user_shop) => {
                user_shop.coupons.push(&Coupon::new(code, discount_percentage));
            }
            None => {
                env::panic_str("You need to register your shop before adding coupons");
            }
        }
    }

    fn add_specific_coupon(&mut self, code: String, discount_percentage: f32, applies_to_products: &Vec<String>, applies_to_user: Option<AccountId>, is_one_time: bool) {
        let user_shop_maybe = self.user_shops.get(&env::predecessor_account_id());
        match user_shop_maybe {
            Some(mut user_shop) => {
                user_shop.coupons.push(&Coupon::specific_new(code, discount_percentage, applies_to_products, applies_to_user, is_one_time));
            }
            None => {
                env::panic_str("You need to register your shop before adding coupons");
            }
        }
    }
}

impl NearShop {
    fn convert_products(&self, user_shop: &UserShop, products: &Vec<String>) -> Vec<ProductDto> {
        let products = user_shop.products.to_vec().intersect_with_ids(|x: &Product| String::from(&x.id), products, |left, right| left == right);
        let mut return_value = Vec::new();
        for product in products {
            return_value.push(ProductDto::new(String::from(&product.id), String::from(&product.name), product.price));
        }

        return_value
    }
}

