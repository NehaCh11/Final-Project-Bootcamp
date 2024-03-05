// src/main.rs

mod models;
mod operations;
mod reports;
mod ui;

fn main() {
    let mut products: Vec<models::Product> = Vec::new();
    ui::start_ui(&mut products);
}
