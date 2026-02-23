use std::io::{self, Write};
use std::str::FromStr;

fn get_input<T: FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("⚠️ Invalid input. Please try again."),
        }
    }
}

#[derive(Debug)]
enum Category {
    Food,
    Electronic,
    Clothing,
}

#[derive(Debug)]
struct Product {
    name: String,
    price: f32,
    category: Category,
    number_of_items: u32,
}

impl Product {
    fn total_value(&self) -> f32 {
        self.price * (self.number_of_items as f32)
    }
}

fn main() {
    let mut warehouse: Vec<Product> = Vec::new();
    
    loop {
        println!("\n📦 --- Warehouse Management System ---");
        println!("1. Add Product");
        println!("2. List All Products");
        println!("3. Search Product");
        println!("4. Exit");

        let choice = get_input::<i32>("👉 Enter your choice: ");

        match choice {
            1 => {
                let name = get_input::<String>("Enter product name: ");
                let price = get_input::<f32>("Enter product price: ");
                let category_str = get_input::<String>("Enter category (Food/Electronic/Clothing): ");
                let number_of_items = get_input::<u32>("Enter quantity: ");

                let category = match category_str.trim().to_lowercase().as_str() {
                    "food" => Category::Food,
                    "electronic" => Category::Electronic,
                    "clothing" => Category::Clothing,
                    _ => {
                        println!("❌ Invalid category. Product not added.");
                        continue;
                    }
                };

                warehouse.push(Product {
                    name,
                    price,
                    category,
                    number_of_items,
                });
                println!("✅ Product added successfully!");
            }
            2 => {
                if warehouse.is_empty() {
                    println!("📭 Warehouse is empty!");
                    continue;
                }
                display_table(&warehouse);
            }
            3 => {
                if warehouse.is_empty() {
                    println!("📭 Nothing to search. Warehouse is empty!");
                    continue;
                }
                let query = get_input::<String>("🔍 Enter product name to search: ");
                let filtered: Vec<&Product> = warehouse.iter()
                    .filter(|p| p.name.to_lowercase().contains(&query.to_lowercase()))
                    .collect();

                if filtered.is_empty() {
                    println!("Searching for '{}'... No results found.", query);
                } else {
                    println!("\n🔎 Search Results for '{}':", query);
                    display_filtered_table(filtered);
                }
            }
            4 => {
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("🚫 Invalid choice!"),
        }
    }
}

fn display_table(products: &Vec<Product>) {
    println!("\n{:<3} | {:<12} | {:<12} | {:<8} | {:<5} | {:<12}", 
             "ID", "Name", "Category", "Price", "Qty", "Total Value");
    println!("{:-<70}", ""); 
    for (i, p) in products.iter().enumerate() {
        println!("{:<3} | {:<12} | {:<12?} | {:<8.2} | {:<5} | {:<12.2}",
                 i + 1, p.name, p.category, p.price, p.number_of_items, p.total_value());
    }
}

fn display_filtered_table(products: Vec<&Product>) {
    println!("\n{:<12} | {:<12} | {:<8} | {:<5} | {:<12}", 
             "Name", "Category", "Price", "Qty", "Total Value");
    println!("{:-<60}", ""); 
    for p in products {
        println!("{:<12} | {:<12?} | {:<8.2} | {:<5} | {:<12.2}",
                 p.name, p.category, p.price, p.number_of_items, p.total_value());
    }
}