use crate::dto::coupon::CouponDto;
use crate::dto::product::ProductDto;
use crate::dto::user_shop::UserShopDto;
use near_sdk::{AccountId, Promise};

pub trait NearShopContract {
    fn new() -> Self;

    // Read methods
    fn get_my_user_shop(&self) -> Option<UserShopDto>;
    fn list_all_user_shops(&self) -> Vec<UserShopDto>;
    fn list_user_shop_products(&self, user_shop_id: String) -> Vec<ProductDto>;
    fn get_user_shop_product(&self, user_shop_id: String, product_id: String)
        -> Option<ProductDto>;
    fn list_my_user_shop_coupons(&self) -> Vec<CouponDto>;
    fn get_product_cost_using_coupon(
        &self,
        user_shop_id: String,
        product_id: String,
        quantity: i32,
        coupon_code: String,
    ) -> u128;

    // Write methods
    fn add_user_shop(&mut self, name: String);
    fn add_product(&mut self, name: String, price: u128, quantity: i32);
    fn update_product_quantity(&mut self, product_id: String, quantity: i32);
    fn add_default_coupon(&mut self, code: String, discount_percentage: f32);
    fn add_specific_coupon(
        &mut self,
        code: String,
        discount_percentage: f32,
        applies_to_products: &Vec<String>,
        applies_to_user: Option<AccountId>,
        is_one_time: bool,
    );
    fn buy_product(
        &mut self,
        user_shop_id: String,
        product_id: String,
        quantity: i32,
        using_coupon_code: Option<String>,
    ) -> Promise;
}
