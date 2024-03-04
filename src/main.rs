use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;

// Define the Product struct to hold product details
#[derive(Debug, PartialEq, Clone)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: i32,
}

// Define the Inventory struct to manage products
struct Inventory {
    products: HashMap<String, Product>,
}

impl Inventory {
    // Constructor for Inventory
    fn new() -> Inventory {
        Inventory {
            products: HashMap::new(),
        }
    }

    // Method to add a product to the inventory
    fn add_product(&mut self, product: Product) {
        self.products.insert(product.name.clone(), product);
    }

    // Method to edit a product in the inventory
    fn edit_product(&mut self, name: &str, product: Product) -> Result<(), String> {
        if let Some(existing_product) = self.products.get_mut(name) {
            *existing_product = product;
            Ok(())
        } else {
            Err(String::from("Product not found"))
        }
    }

    // Method to delete a product from the inventory
    fn delete_product(&mut self, name: &str) -> Result<(), String> {
        if self.products.remove(name).is_some() {
            Ok(())
        } else {
            Err(String::from("Product not found"))
        }
    }

    // Method to generate inventory report
    fn generate_report(&self) {
        println!("Inventory Report:");
        println!("-----------------");
        for product in self.products.values() {
            println!(
                "Name: {}, Description: {}, Price: {}, Quantity: {}",
                product.name, product.description, product.price, product.quantity
            );
        }
    }

    // Method to load inventory data from a file
    fn load_from_file(filename: &str) -> Result<Inventory, String> {
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(_) => return Err(String::from("Unable to open file")),
        };

        let reader = BufReader::new(file);
        let mut inventory = Inventory::new();

        for line in reader.lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => return Err(String::from("Error reading file")),
            };
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 4 {
                return Err(String::from("Invalid file format"));
            }
            let name = String::from(parts[0]);
            let description = String::from(parts[1]);
            let price = match parts[2].parse::<f64>() {
                Ok(price) => price,
                Err(_) => return Err(String::from("Invalid price")),
            };
            let quantity = match parts[3].parse::<i32>() {
                Ok(quantity) => quantity,
                Err(_) => return Err(String::from("Invalid quantity")),
            };
            let product = Product {
                name,
                description,
                price,
                quantity,
            };
            inventory.add_product(product);
        }

        Ok(inventory)
    }

    // Method to save inventory data to a file
    fn save_to_file(&self, filename: &str) -> Result<(), String> {
        let mut file = match File::create(filename) {
            Ok(file) => file,
            Err(_) => return Err(String::from("Unable to create file")),
        };

        for product in self.products.values() {
            let line = format!(
                "{},{},{},{}\n",
                product.name, product.description, product.price, product.quantity
            );
            if let Err(_) = file.write_all(line.as_bytes()) {
                return Err(String::from("Error writing to file"));
            }
        }

        Ok(())
    }
}

// Function to authenticate users
fn authenticate() -> bool {
    println!("Please enter your username and password.");
    print!("Username: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    print!("Password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    // Basic authentication logic (for demonstration purposes)
    username.trim() == "azfar" && password.trim() == "organic"
}

fn main() {
    if !authenticate() {
        println!("Authentication failed. Exiting...");
        return;
    }

    let inventory_filename = "inventory.txt";

    // Load inventory from file or create a new one
    let mut inventory = match Inventory::load_from_file(inventory_filename) {
        Ok(inventory) => inventory,
        Err(_) => {
            println!("Creating a new inventory.");
            Inventory::new()
        }
    };

    loop {
        println!("\nMenu:");
        println!("1. Add Product");
        println!("2. Edit Product");
        println!("3. Delete Product");
        println!("4. Generate Report");
        println!("5. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter product details:");
                print!("Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");

                print!("Description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");

                print!("Price: ");
                io::stdout().flush().unwrap();
                let mut price = String::new();
                io::stdin().read_line(&mut price).expect("Failed to read line");
                let price: f64 = price.trim().parse().expect("Invalid price");

                print!("Quantity: ");
                io::stdout().flush().unwrap();
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).expect("Failed to read line");
                let quantity: i32 = quantity.trim().parse().expect("Invalid quantity");

                let product = Product {
                    name: name.trim().to_string(),
                    description: description.trim().to_string(),
                    price,
                    quantity,
                };

                inventory.add_product(product);
            }
            2 => {
                println!("Enter product name to edit:");
                print!("Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");

                if let Some(product) = inventory.products.get(&name.trim().to_string()) {
                    println!("Enter new product details:");
                    print!("Description: ");
                    io::stdout().flush().unwrap();
                    let mut description = String::new();
                    io::stdin().read_line(&mut description).expect("Failed to read line");

                    print!("Price: ");
                    io::stdout().flush().unwrap();
                    let mut price = String::new();
                    io::stdin().read_line(&mut price).expect("Failed to read line");
                    let price: f64 = price.trim().parse().expect("Invalid price");

                    print!("Quantity: ");
                    io::stdout().flush().unwrap();
                    let mut quantity = String::new();
                    io::stdin().read_line(&mut quantity).expect("Failed to read line");
                    let quantity: i32 = quantity.trim().parse().expect("Invalid quantity");

                    let edited_product = Product {
                        name: name.trim().to_string(),
                        description: description.trim().to_string(),
                        price,
                        quantity,
                    };

                    if let Err(err) = inventory.edit_product(&name.trim(), edited_product) {
                        println!("Error: {}", err);
                    }
                } else {
                    println!("Product not found");
                }
            }
            3 => {
                println!("Enter product name to delete:");
                print!("Name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");

                if let Err(err) = inventory.delete_product(&name.trim()) {
                    println!("Error: {}", err);
                }
            }
            4 => {
                inventory.generate_report();
            }
            5 => {
                // Save inventory to file before exiting
                if let Err(err) = inventory.save_to_file(inventory_filename) {
                    println!("Error saving inventory: {}", err);
                }
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a sample inventory for testing
    fn create_sample_inventory() -> Inventory {
        let mut inventory = Inventory::new();
        let products = vec![
            Product {
                name: "Product1".to_string(),
                description: "Description1".to_string(),
                price: 10.0,
                quantity: 5,
            },
            Product {
                name: "Product2".to_string(),
                description: "Description2".to_string(),
                price: 20.0,
                quantity: 10,
            },
        ];
        for product in products {
            inventory.add_product(product);
        }
        inventory
    }

    #[test]
    fn test_add_product() {
        let mut inventory = Inventory::new();
        let product = Product {
            name: "TestProduct".to_string(),
            description: "TestDescription".to_string(),
            price: 15.0,
            quantity: 8,
        };
        inventory.add_product(product.clone());
        assert_eq!(inventory.products.len(), 1);
        assert_eq!(inventory.products.get("TestProduct"), Some(&product));
    }

    #[test]
    fn test_edit_product() {
        let mut inventory = create_sample_inventory();
        let edited_product = Product {
            name: "Product1".to_string(),
            description: "NewDescription".to_string(),
            price: 12.0,
            quantity: 20,
        };
        inventory.edit_product("Product1", edited_product.clone()).unwrap();
        assert_eq!(
            inventory.products.get("Product1"),
            Some(&edited_product)
        );
    }

    #[test]
    fn test_edit_non_existing_product() {
        let mut inventory = create_sample_inventory();
        let edited_product = Product {
            name: "NonExistingProduct".to_string(),
            description: "NewDescription".to_string(),
            price: 12.0,
            quantity: 20,
        };
        let result = inventory.edit_product("NonExistingProduct", edited_product);
        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some(String::from("Product not found"))
        );
    }

    #[test]
    fn test_delete_product() {
        let mut inventory = create_sample_inventory();
        inventory.delete_product("Product1").unwrap();
        assert_eq!(inventory.products.len(), 1);
        assert!(inventory.products.get("Product1").is_none());
    }

    #[test]
    fn test_delete_non_existing_product() {
        let mut inventory = create_sample_inventory();
        let result = inventory.delete_product("NonExistingProduct");
        assert!(result.is_err());
        assert_eq!(
            result.err(),
            Some(String::from("Product not found"))
        );
    }

/*     #[test]
    fn test_generate_report() {
        let inventory = create_sample_inventory();
        // Redirect stdout to capture printed output
        let mut output = Vec::new();
        let old_stdout = std::io::stdout();
        std::io::stdout().lock().set_buffer(output);
        inventory.generate_report();
        // Restore stdout
        std::io::stdout().lock().set_buffer(old_stdout.lock().buffer());
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Product1"));
        assert!(output_str.contains("Product2"));
    } */

    #[test]
    fn test_load_and_save_to_file() {
        let filename = "test_inventory.txt";
        let mut inventory = create_sample_inventory();
        inventory
            .save_to_file(filename)
            .expect("Error saving to file");
        let loaded_inventory =
            Inventory::load_from_file(filename).expect("Error loading from file");
        assert_eq!(inventory.products, loaded_inventory.products);
        // Clean up the created file
        std::fs::remove_file(filename).expect("Error removing file");
    }
}
