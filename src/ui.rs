// src/ui.rs

use crate::models::Product;
use crate::operations::{add_product, delete_product, edit_product};
use crate::reports::generate_report;
use std::io::{self, Write};

pub fn start_ui(products: &mut Vec<Product>) {
    loop {
        println!("Inventory Management System:");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Generate Report");
        println!("5. Exit");

        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = choice.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                // Code to add product
                let product = Product {
                    name: "Example".to_string(),
                    description: "Example product".to_string(),
                    price: 19.99,
                    quantity: 10,
                };
                add_product(products, product);
                println!("Product added.");
            },
            2 => {
                // Code to edit product
                let updated_product = Product {
                    name: "Example Updated".to_string(),
                    description: "Updated example product".to_string(),
                    price: 29.99,
                    quantity: 5,
                };
                edit_product(products, "Example", updated_product);
                println!("Product updated.");
            },
            3 => {
                // Code to delete product
                delete_product(products, "Example Updated");
                println!("Product deleted.");
            },
            4 => {
                // Generate and print report
                generate_report(&products);
            },
            5 => break,
            _ => println!("Invalid choice. Please enter 1-5."),
        }
    }
}
