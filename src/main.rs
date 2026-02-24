use std::io::{self, Write};
use std::str::FromStr;
use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize, Debug)]
enum Category {
    Food,
    Electronic,
    Clothing,
}

#[derive(Serialize, Deserialize, Debug)]
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
    let mut warehouse: Vec<Product> = load_from_csv();
    
    loop {
        println!("\n📦 --- Warehouse Management System ---");
        println!("1. Add Product");
        println!("2. List All Products");
        println!("3. Search Product");
        println!("4. Save and Exit");

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
                match save_to_csv(&warehouse) {
                    Ok(_) => println!("💾 Data saved to inventory.csv successfully!"),
                    Err(e) => println!("❌ Error saving data: {}", e),
                }
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("🚫 Invalid choice!"),
        }
    }
}

fn display_table(products: &Vec<Product>) {
    println!("\n{:-<69}", "");
    println!("{:<3} | {:<12} | {:<12} | {:>10} | {:>5} | {:>12}", 
             "ID", "Name", "Category", "Price", "Qty", "Total");
    println!("{:-<69}", "");

    for (i, p) in products.iter().enumerate() {
        let cat_str = format!("{:?}", p.category);
        let name_str = if p.name.len() > 12 { p.name[..12].to_string() } else { p.name.clone() };
        
        println!("{:<3} | {:<12} | {:<12} | {:>10.2} | {:>5} | {:>12.2}",
                 i + 1, name_str, cat_str, p.price, p.number_of_items, p.total_value());
    }
    println!("{:-<69}", "");
}

fn display_filtered_table(products: Vec<&Product>) {
    println!("\n{:-<62}", "");
    println!("{:<12} | {:<12} | {:>10} | {:>5} | {:>12}", 
             "Name", "Category", "Price", "Qty", "Total");
    println!("{:-<62}", "");

    for p in products {
        let cat_str = format!("{:?}", p.category);
        let name_str = if p.name.len() > 12 { p.name[..12].to_string() } else { p.name.clone() };

        println!("{:<12} | {:<12} | {:>10.2} | {:>5} | {:>12.2}",
                 name_str, cat_str, p.price, p.number_of_items, p.total_value());
    }
    println!("{:-<62}", "");
}

fn save_to_csv(products: &Vec<Product>) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = csv::Writer::from_path("inventory.csv")?;
    for product in products {
        writer.serialize(product)?;
    }
    writer.flush()?;
    Ok(())
}

fn load_from_csv() -> Vec<Product> {
    let mut products = Vec::new();
    if let Ok(mut reader) = csv::Reader::from_path("inventory.csv") {
        for result in reader.deserialize() {
            if let Ok(product) = result {
                products.push(product);
            }
        }
        println!("📂 Data loaded from inventory.csv successfully!");
    } else {
        println!("ℹ️ No existing inventory file found. Starting fresh.");
    }
    products
}