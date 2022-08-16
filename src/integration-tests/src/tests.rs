use anyhow::Error;
use near_sdk::json_types::U128;
use near_units::parse_near;
use serde_json::json;
use shop::dto::product::ProductDto;
use shop::dto::user_shop::UserShopDto;
use std::{env, fs};
use workspaces::prelude::*;
use workspaces::{network::Sandbox, Account, Contract, Worker};

const WASM_FILEPATH: &str = "../contract/target/wasm32-unknown-unknown/release/shop.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let wasm_arg = WASM_FILEPATH;
    let wasm_filepath = fs::canonicalize(env::current_dir()?.join(wasm_arg))?;

    let worker = workspaces::sandbox().await?;
    let wasm = std::fs::read(wasm_filepath)?;
    let contract = worker.dev_deploy(&wasm).await?;

    // create accounts
    let account = worker.dev_create_account().await?;
    let alice = account
        .create_subaccount(&worker, "alice")
        .initial_balance(parse_near!("30 N"))
        .transact()
        .await?
        .into_result()?;

    test_created_shop_with_two_products(&alice, &contract, &worker).await?;
    Ok(())
}

async fn test_created_shop_with_two_products(
    user: &Account,
    contract: &Contract,
    worker: &Worker<Sandbox>,
) -> anyhow::Result<()> {
    // Initialize the contract
    user.call(&worker, contract.id(), "initialize")
        .transact()
        .await?;

    // Get the user shop
    let mut user_shop: Result<UserShopDto, Error> = user
        .call(&worker, contract.id(), "get_my_user_shop")
        .transact()
        .await?
        .json();

    // If it doesn't exist, create one
    if user_shop.is_err() {
        user.call(&worker, contract.id(), "add_user_shop")
            .args_json(json!({ "name": "Alice"}))?
            .transact()
            .await?;

        user_shop = user
            .call(&worker, contract.id(), "get_my_user_shop")
            .transact()
            .await?
            .json::<UserShopDto>();
    }

    let unwrapped_user_shop = user_shop?;

    // Assert the shop is created successfully
    assert_eq!("Alice", unwrapped_user_shop.name);

    // Create two products
    user.call(&worker, contract.id(), "add_product")
        .args_json(json!({ "name": "Product 1", "price": U128(100000), "quantity": 10}))?
        .transact()
        .await?;

    user.call(&worker, contract.id(), "add_product")
        .args_json(json!({ "name": "Product 2", "price": U128(200000), "quantity": 5}))?
        .transact()
        .await?;

    let products: Vec<ProductDto> = user
        .call(&worker, contract.id(), "list_user_shop_products")
        .args_json(json!({ "user_shop_id": unwrapped_user_shop.id}))?
        .transact()
        .await?
        .json()?;

    assert_eq!(2, products.len());

    let product1 = &products[0];
    let product2 = &products[1];

    assert_eq!("Product 1".to_string(), product1.name);
    assert_eq!("Product 2".to_string(), product2.name);

    assert_eq!(U128(100000), U128(product1.price));
    assert_eq!(U128(200000), U128(product2.price));

    assert_eq!(10, product1.quantity_on_stock);
    assert_eq!(5, product2.quantity_on_stock);
    println!("      Passed ✅ Created two products");
    Ok(())
}
