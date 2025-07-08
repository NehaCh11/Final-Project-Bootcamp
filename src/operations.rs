// src/operations.rs

use crate::models::Product;

pub fn add_product(products: &mut Vec<Product>, new_product: Product) {
    products.push(new_product);
}

pub fn edit_product(products: &mut Vec<Product>, name: &str, updated_product: Product) {
    let pos = products.iter().position(|p| p.name == name);
    if let Some(pos) = pos {
        products[pos] = updated_product;
    }
}

pub fn delete_product(products: &mut Vec<Product>, name: &str) {
    products.retain(|product| product.name != name);
}


