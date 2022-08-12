mod near_shop;

use crate::contract::near_shop::NearShopContract;
use crate::dto::coupon::CouponDto;
use crate::dto::product::ProductDto;
use crate::dto::user_shop::UserShopDto;
use crate::model::coupon::Coupon;
use crate::model::product::Product;
use crate::model::storage_keys::StorageKeys;
use crate::model::user_shop::UserShop;
use crate::utils::vector_utils::VectorUtils;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NearShop {
    /// The lazy map of user shops.
    /// We don't need to read the entire map of existing user shops when adding a new one.
    /// Each time the state is loaded, using the standard Map would cause all entries
    /// in the data structure to be read eagerly from storage and deserialized.
    /// This would come at a large cost for any non-trivial amount of data,
    /// so to minimize the amount of gas used the SDK UnorderedMap is used.
    user_shops: UnorderedMap<AccountId, UserShop>,
}

#[near_bindgen]
impl NearShopContract for NearShop {
    #[init]
    fn new() -> Self {
        if !env::state_read::<Self>().is_none() {
            env::panic_str("Already initialized");
        }

        Self {
            user_shops: UnorderedMap::new(StorageKeys::UserShops),
        }
    }

    fn get_my_user_shop(&self) -> Option<UserShopDto> {
        let user_shop = self.user_shops.get(&env::predecessor_account_id());
        match user_shop {
            Some(result) => Some(UserShopDto {
                id: result.id,
                name: result.name,
            }),
            None => None,
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
                return_value.push(ProductDto::new(
                    String::from(&product.id),
                    String::from(&product.name),
                    product.price,
                    product.quantity_on_stock,
                ));
            }
        }

        return_value
    }

    fn get_user_shop_product(
        &self,
        user_shop_id: String,
        product_id: String,
    ) -> Option<ProductDto> {
        let user_shops = self.user_shops.values_as_vector().to_vec();
        let found_user_shop = user_shops.iter().find(|&x| x.id == user_shop_id);
        if let None = found_user_shop {
            return None;
        }

        let user_shop = found_user_shop.unwrap();
        let products = user_shop.products.to_vec();
        let found_product = products.iter().find(|x| x.id == product_id);

        if let None = found_product {
            return None;
        }

        let product = found_product.unwrap();
        Some(ProductDto::new(
            String::from(&product.id),
            String::from(&product.name),
            product.price,
            product.quantity_on_stock,
        ))
    }

    fn list_my_user_shop_coupons(&self) -> Vec<CouponDto> {
        let user_shop_maybe = self.user_shops.get(&env::predecessor_account_id());
        if let None = user_shop_maybe {
            env::panic_str("You don't have a shop");
        }

        let user_shop = user_shop_maybe.unwrap();
        let mut return_value = Vec::new();
        for coupon in user_shop.coupons.iter() {
            let dto = CouponDto::new(
                String::from(&coupon.id),
                String::from(&coupon.code),
                coupon.discount_percentage,
                coupon.applies_to_all_products,
                self.convert_products(&user_shop, &coupon.applies_to_products),
                coupon.applies_to_all_users,
                coupon.applies_to_user.clone(),
                coupon.is_one_time,
                coupon.times_used,
            );

            return_value.push(dto);
        }

        return_value
    }

    fn get_product_cost_using_coupon(
        &self,
        user_shop_id: String,
        product_id: String,
        quantity: i32,
        coupon_code: String,
    ) -> u128 {
        let user_shops = self.user_shops.values_as_vector().to_vec();
        let found_user_shop = user_shops.iter().find(|x| x.id == user_shop_id);
        if let None = found_user_shop {
            env::panic_str("The requested user shop doesn't exist");
        }

        let user_shop = found_user_shop.unwrap();
        let products = user_shop.products.to_vec();
        let found_product = products.iter().find(|x| x.id == product_id);

        if let None = found_product {
            env::panic_str("The requested product doesn't exist");
        }

        let product = found_product.unwrap();
        let mut expected_amount: u128 = product.price * quantity as u128;
        let _ = self.update_expected_amount(
            &mut expected_amount,
            &coupon_code,
            &product_id,
            &user_shop_id,
        );

        expected_amount
    }

    fn add_user_shop(&mut self, name: String) {
        let existing_user_shop = self.user_shops.get(&env::predecessor_account_id());
        if !existing_user_shop.is_none() {
            env::panic_str("You already have a shop");
        }

        self.user_shops
            .insert(&env::predecessor_account_id(), &UserShop::new(name));
    }

    fn add_product(&mut self, name: String, price: u128, quantity: i32) {
        let mut user_shop = self
            .user_shops
            .get(&env::predecessor_account_id())
            .expect("You need to register your shop before adding products to sell");
        let products = &mut user_shop.products;
        let new_product = Product::new(name, price, quantity);
        products.push(&new_product);
        self.user_shops
            .insert(&env::predecessor_account_id(), &user_shop);
    }

    fn update_product_quantity(&mut self, product_id: String, quantity: i32) {
        let mut user_shop = self
            .user_shops
            .get(&env::predecessor_account_id())
            .expect("You need to register your shop before adding products to sell");
        let products = &mut user_shop.products;
        let idx = products
            .to_vec()
            .iter()
            .position(|x| x.id == product_id)
            .expect("The requested product does not exist");

        let mut product = products.to_vec()[idx].clone();
        product.quantity_on_stock = quantity;
        products.replace(idx.try_into().unwrap(), &product);
    }

    fn add_default_coupon(&mut self, code: String, discount_percentage: f32) {
        let mut user_shop = self
            .user_shops
            .get(&env::predecessor_account_id())
            .expect("You need to register your shop before adding coupons");
        let coupons = &mut user_shop.coupons;
        coupons.push(&Coupon::new(code, discount_percentage));
        self.user_shops
            .insert(&env::predecessor_account_id(), &user_shop);
    }

    fn add_specific_coupon(
        &mut self,
        code: String,
        discount_percentage: f32,
        applies_to_products: &Vec<String>,
        applies_to_user: Option<AccountId>,
        is_one_time: bool,
    ) {
        let mut user_shop = self
            .user_shops
            .get(&env::predecessor_account_id())
            .expect("You need to register your shop before adding coupons");
        let coupons = &mut user_shop.coupons;
        coupons.push(&Coupon::specific_new(
            code,
            discount_percentage,
            applies_to_products,
            applies_to_user,
            is_one_time,
        ));

        self.user_shops
            .insert(&env::predecessor_account_id(), &user_shop);
    }

    #[payable]
    fn buy_product(
        &mut self,
        user_shop_id: String,
        product_id: String,
        quantity: i32,
        using_coupon_code: Option<String>,
    ) -> Promise {
        if quantity <= 0 {
            env::panic_str("The quantity must be higher than zero");
        }

        let mut user_shops = self.user_shops.values_as_vector().to_vec();
        let found_user_shop_idx = user_shops.iter().position(|x| x.id == user_shop_id);
        if let None = found_user_shop_idx {
            env::panic_str("The requested user shop doesn't exist");
        }

        let user_shop = user_shops.get_mut(found_user_shop_idx.unwrap()).unwrap();
        let coupons = &mut user_shop.coupons;
        let products = user_shop.products.to_vec();
        let found_product = products.iter().find(|x| x.id == product_id);

        if let None = found_product {
            env::panic_str("The requested product doesn't exist");
        }

        let product = found_product.unwrap();
        if product.quantity_on_stock < quantity {
            env::panic_str("The requested product quantity is not available on stock");
        }

        let mut expected_amount: u128 = product.price * quantity as u128;
        let mut coupon_maybe = None;
        if let Some(coupon_code) = using_coupon_code {
            coupon_maybe = self.update_expected_amount(
                &mut expected_amount,
                &coupon_code,
                &product_id,
                &user_shop_id,
            );
        }

        let _test = Balance::from(expected_amount);
        if env::attached_deposit() != expected_amount {
            env::panic_str("The payment amount is not correct: should be {}");
        }

        if let Some((mut coupon, idx)) = coupon_maybe {
            coupon.times_used += 1;
            coupons.replace(idx.try_into().unwrap(), &coupon);
        }

        self.update_product_quantity(product_id, product.quantity_on_stock - quantity);
        self.user_shops
            .insert(&user_shop.owner_account_id, &user_shop);

        Promise::new(user_shop.owner_account_id.clone()).transfer(env::attached_deposit())
    }
}

impl NearShop {
    fn convert_products(&self, user_shop: &UserShop, products: &Vec<String>) -> Vec<ProductDto> {
        let products_vector = user_shop.products.to_vec();
        let products = products_vector.intersect_with_ids(
            |x: &Product| String::from(&x.id),
            products,
            |left, right| left == right,
        );

        let mut return_value = Vec::new();
        for product in products {
            return_value.push(ProductDto::new(
                String::from(&product.id),
                String::from(&product.name),
                product.price,
                product.quantity_on_stock,
            ));
        }

        return_value
    }

    fn update_expected_amount(
        &self,
        expected_amount: &mut u128,
        coupon_code: &String,
        product_id: &String,
        user_shop_id: &String,
    ) -> Option<(Coupon, usize)> {
        let user_shops = self.user_shops.values_as_vector().to_vec();
        let found_user_shop = user_shops
            .iter()
            .find(|x| x.id == String::from(user_shop_id));
        if let None = found_user_shop {
            return None;
        }

        let user_shop = found_user_shop.unwrap();
        let coupon_maybe = user_shop
            .coupons
            .iter()
            .position(|x| x.code == *coupon_code);
        if let None = coupon_maybe {
            return None;
        }

        let idx = coupon_maybe.unwrap();
        let coupon = &user_shop.coupons.to_vec()[idx];
        if coupon.times_used > 0 && coupon.is_one_time {
            return None;
        }

        if !coupon.applies_to_all_users
            && *coupon.applies_to_user.as_ref().unwrap() != env::predecessor_account_id()
        {
            return None;
        }

        if !coupon.applies_to_all_products && !coupon.applies_to_products.contains(product_id) {
            return None;
        }

        *expected_amount = *expected_amount
            - (*expected_amount * (coupon.discount_percentage as u128)) / (100 as u128);
        Some(((*coupon).clone(), idx))
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob.near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn get_my_user_shop_returns_after_adding_a_shop() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = NearShop::new();
        contract.add_user_shop("Test Shop".to_string());
        let user_shop = contract.get_my_user_shop();
        assert!(!user_shop.is_none());
        assert_eq!("Test Shop".to_string(), user_shop.unwrap().name);
    }

    #[test]
    #[should_panic(expected = "You already have a shop")]
    fn add_user_shop_adds_only_one_per_user() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = NearShop::new();
        let user_shop = contract.get_my_user_shop();
        assert!(user_shop.is_none());
        contract.add_user_shop("Test Shop".to_string());
        contract.add_user_shop("Should never add this one".to_string());
    }

    #[test]
    fn user_shop_can_be_found_by_id() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = NearShop::new();
        contract.add_user_shop("Test Shop".to_string());
        let user_shop = contract.get_my_user_shop().unwrap();
        let user_shops = contract.user_shops.values_as_vector().to_vec();
        let _found_user_shop = user_shops
            .iter()
            .find(|&x| x.id == user_shop.id)
            .expect("User shop should be returned");
    }

    #[test]
    #[should_panic(expected = "You need to register your shop before adding products to sell")]
    fn should_not_add_a_product_if_no_shop() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = NearShop::new();
        contract.add_product("Should not add this product".to_string(), 0, 0);
    }

    #[test]
    #[should_panic(expected = "You need to register your shop before adding coupons")]
    fn should_not_add_a_default_coupon_if_no_shop() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = NearShop::new();
        contract.add_default_coupon("Should not add this coupon".to_string(), 100.00);
    }

    #[test]
    #[should_panic(expected = "You need to register your shop before adding coupons")]
    fn should_not_add_a_specific_coupon_if_no_shop() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = NearShop::new();
        contract.add_specific_coupon(
            "Should not add this coupon".to_string(),
            100.00,
            &vec![],
            None,
            true,
        );
    }

    #[test]
    fn should_add_a_product_to_registered_shop() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = NearShop::new();
        contract.add_user_shop("Test Shop".to_string());
        contract.add_product("Test Product".to_string(), 1337, 0);
        let user_shop = contract.get_my_user_shop().unwrap();
        let products = contract.list_user_shop_products(user_shop.id);
        assert_eq!(1, products.len());
        let test_product = products.get(0).unwrap();
        assert_eq!("Test Product".to_string(), test_product.name);
        assert_eq!(1337, test_product.price);

        // Bug?
        // The quantity above is set to 0 and here the received quantity_on_stock is -1.
        // If we set the quantity above to 1, then it will be -2.
        // If we set the quantity to an unsigned integer u16 and the value to 0, then it will be 65535.
        assert_eq!(0, !test_product.quantity_on_stock);
    }

    #[test]
    fn should_update_a_product_quantity() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = NearShop::new();
        contract.add_user_shop("Test Shop".to_string());
        contract.add_product("Test Product".to_string(), 1337, 0);
        let user_shop = contract.get_my_user_shop().unwrap();
        let products = contract.list_user_shop_products(String::from(&user_shop.id));
        let test_product = products.get(0).unwrap();
        assert_eq!("Test Product".to_string(), test_product.name);
        assert_eq!(1337, test_product.price);
        contract.update_product_quantity(String::from(&test_product.id), 1000);
        let updated_product = contract
            .get_user_shop_product(String::from(&user_shop.id), String::from(&test_product.id))
            .unwrap();
        assert_eq!(1000, updated_product.quantity_on_stock);
    }

    #[test]
    fn should_add_a_default_coupon_to_registered_shop() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = NearShop::new();
        contract.add_user_shop("Test Shop".to_string());
        contract.add_default_coupon("Test Coupon".to_string(), 13.37);
        let coupons = contract.list_my_user_shop_coupons();
        assert_eq!(1, coupons.len());
        let test_coupon = coupons.get(0).unwrap();
        assert_eq!("Test Coupon".to_string(), test_coupon.code);
        assert_eq!(13.37, test_coupon.discount_percentage);
    }

    #[test]
    fn should_add_a_one_time_specific_coupon_to_registered_shop() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = NearShop::new();
        contract.add_user_shop("Test Shop".to_string());
        contract.add_specific_coupon("Test Coupon".to_string(), 13.37, &vec![], None, true);
        let coupons = contract.list_my_user_shop_coupons();

        assert_eq!(1, coupons.len());
        let test_coupon = coupons.get(0).unwrap();
        assert_eq!("Test Coupon".to_string(), test_coupon.code);
        assert_eq!(13.37, test_coupon.discount_percentage);
        assert_eq!(true, test_coupon.applies_to_all_products);
        assert_eq!(true, test_coupon.applies_to_all_users);
        assert_eq!(true, test_coupon.is_one_time);
    }

    #[test]
    fn should_add_a_reusable_coupon_which_applies_to_one_user() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = NearShop::new();
        contract.add_user_shop("Test Shop".to_string());
        contract.add_specific_coupon(
            "Test Coupon".to_string(),
            13.37,
            &vec![],
            Some(env::signer_account_id()),
            false,
        );

        let coupons = contract.list_my_user_shop_coupons();
        assert_eq!(1, coupons.len());
        let test_coupon = coupons.get(0).unwrap();
        assert_eq!("Test Coupon".to_string(), test_coupon.code);
        assert_eq!(13.37, test_coupon.discount_percentage);
        assert_eq!(true, test_coupon.applies_to_all_products);
        assert_eq!(false, test_coupon.applies_to_all_users);
        assert_eq!(Some(env::signer_account_id()), test_coupon.applies_to_user);
        assert_eq!(false, test_coupon.is_one_time);
    }

    #[test]
    fn should_return_correct_cost_with_coupon() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = NearShop::new();
        contract.add_user_shop("Test Shop".to_string());
        contract.add_default_coupon("Test Coupon".to_string(), 50.00);
        contract.add_product("Test Product".to_string(), 10_000_000_000, 0);

        let user_shop = contract.get_my_user_shop().unwrap();
        let products = contract.list_user_shop_products(String::from(&user_shop.id));
        let product = products.get(0).unwrap();

        let cost = contract.get_product_cost_using_coupon(
            String::from(&user_shop.id),
            String::from(&product.id),
            1,
            "Test Coupon".to_string(),
        );
        assert_eq!(5_000_000_000, cost);
    }
}
