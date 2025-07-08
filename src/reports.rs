// src/reports.rs

use crate::models::Product;

pub fn generate_report(products: &[Product]) {
    println!("Current Inventory:");
    for product in products {
        println!(
            "{} - {} - ${} - Qty: {}",
            product.name, product.description, product.price, product.quantity
        );
    }
}
