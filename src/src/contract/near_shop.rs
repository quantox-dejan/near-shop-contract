use near_sdk::AccountId;
use crate::dto::coupon::CouponDto;
use crate::dto::product::ProductDto;
use crate::dto::user_shop::UserShopDto;

pub trait NearShopContract {
    fn new() -> Self;

    // Getter methods
    fn get_my_user_shop(&self) -> Option<UserShopDto>;
    fn list_all_user_shops(&self) -> Vec<UserShopDto>;
    fn list_user_shop_products(&self, user_shop_id: String) -> Vec<ProductDto>;
    fn list_my_user_shop_coupons(&self) -> Vec<CouponDto>;

    // Setter methods
    fn add_user_shop(&mut self, name: String);
    fn add_product(&mut self, name: String, price: f64);
    fn add_default_coupon(&mut self, code: String, discount_percentage: f32);
    fn add_specific_coupon(&mut self, code: String, discount_percentage: f32, applies_to_products: &Vec<String>, applies_to_user: Option<AccountId>, is_one_time: bool);
}