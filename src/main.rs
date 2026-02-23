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
            Err(_) => println!("Invalid input. Please try again."),
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
        println!("\n--- Warehouse Management ---");
        println!("1. Add product");
        println!("2. List products");
        println!("3. Exit");

        let choice = get_input::<i32>("Enter your choice: ");

        match choice {
            1 => {
                let name = get_input::<String>("Enter product name: ");
                let price = get_input::<f32>("Enter product price: ");
                let category_str = get_input::<String>("Enter product category (Food/Electronic/Clothing): ");
                let number_of_items = get_input::<u32>("Enter number of items: ");

                let category = match category_str.trim().to_lowercase().as_str() {
                    "food" => Category::Food,
                    "electronic" => Category::Electronic,
                    "clothing" => Category::Clothing,
                    _ => {
                        println!("Invalid category. Product not added.");
                        continue;
                    }
                };

                warehouse.push(Product {
                    name,
                    price,
                    category,
                    number_of_items,
                });
                println!("Product added successfully!");
            }
            2 => {
                if warehouse.is_empty() {
                    println!("Warehouse is empty!");
                    continue;
                }
                println!("\nID | Name       | Category   | Price | Qty | Total Value");
                println!("-------------------------------------------------------");
                println!("\n{:<3} | {:<10} | {:<10} | {:<8} | {:<4} | {:<10}", 
                "ID", "Name", "Category", "Price", "Qty", "Total Value");
                println!("{:-<60}", ""); // رسم خط جداکننده
                for (i, p) in warehouse.iter().enumerate() {
                    println!(
                        "{:<3} | {:<10} | {:<10?} | {:<8.2} | {:<4} | {:<10.2}",
                        i + 1,
                        p.name,
                        p.category,
                        p.price,
                        p.number_of_items,
                        p.total_value()
                    );
                }
            }
            3 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}