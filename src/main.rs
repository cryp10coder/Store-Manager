use std::collections::HashMap;
use std::error::Error;

struct Product {
    name: String,
    description: String,
    price: i32,
    quantity: i32,
}

struct Store {
    products: HashMap<String, Product>,
    sales: Vec<(String, i32, i32)>,
    purchases: Vec<(String, i32, i32)>,
}

impl Store {
    fn new() -> Store {
        println!("======    WELCOME TO RUSTY STORE MANAGEMENT SOFTWARE    ======");
        Store {
            products: HashMap::new(),
            sales: Vec::new(),
            purchases: Vec::new(),
        }
    }

    fn add_product(
        &mut self,
        name: String,
        description: String,
        price: i32,
        quantity: i32,
    ) -> Result<(), Box<dyn Error>> {
        if self.products.contains_key(&name) {
            return Err("Product already exists".to_string().into());
        }
        self.products.insert(
            name.clone(),
            Product {
                name,
                description,
                price,
                quantity,
            },
        );
        Ok(())
    }

    fn edit_product(&mut self,name: String,description: String,price: i32,quantity: i32,) -> Result<(), Box<dyn Error>> {
        if let Some(product) = self.products.get_mut(&name) {
            product.description = description;
            product.price = price;
            product.quantity = quantity;
            Ok(())
        } else {
            Err("Product not found".to_string().into())
        }
    }

    fn delete_product(&mut self, name: String) -> Result<(), Box<dyn Error>> {
        if self.products.remove(&name).is_none() {
            return Err("Product not found".to_string().into());
        }
        Ok(())
    }

    fn record_sale(
        &mut self,
        name: String,
        quantity: i32,
        price: i32,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(product) = self.products.get_mut(&name) {
            if product.quantity < quantity {
                return Err("Not enough stock".to_string().into());
            }
            product.quantity -= quantity;
            self.sales.push((name, quantity, price));
            Ok(())
        } else {
            Err("Product not found".to_string().into())
        }
    }

    fn record_purchase(
        &mut self,
        name: String,
        quantity: i32,
        price: i32,
    ) -> Result<(), Box<dyn Error>> {
        if let Some(product) = self.products.get_mut(&name) {
            product.quantity += quantity;
            self.purchases.push((name, quantity, price));
            Ok(())
        } else {
            Err("Product not found".to_string().into())
        }
    }

    fn generate_inventory_report(&self) -> String {

        let mut report = "Inventory Report: \n".to_string();
        report += &format!("{:<20} {:<40} {:<20} {:<20}\n", "NAME", "DESCRIPTION","PRICE","QUANTITY");
        for (_, product) in &self.products{
            report += &format!("{:<20} {:<40} {:<20} {:<20}\n", product.name, product.description, product.price, product.quantity);
        }
        report
    }

    fn generate_sales_report(&self) -> String {
        let mut report = "Sales Report:\n".to_string();
    
        report += &format!("{:<20} {:<20} {:<20} {:<20}\n", "Name", "Quantity", "Price", "Total");
    
        for (name, quantity, price) in &self.sales {
            let total = quantity * price;
            report += &format!("{:<20} {:<20} {:<20} {:<20}\n", name, quantity, price, total);
        }
        report
    }
    
    fn generate_purchases_report(&self) -> String {
        let mut report = "Purchases Report:\n".to_string();
        report += &format!("{:<20} {:<20} {:<20} {:<20}\n", "Name", "Description", "Quantity", "Price");
        for (name, quantity, price) in &self.purchases {
            let product = match self.products.get(name) {
                Some(p) => p,
                None => {
                    continue;
                }
            };
                report += &format!("{:<20} {:<20} {:<20} {:<20}\n", product.name, product.description, quantity, price);
        }
        report
    }
}

fn main() {
    let mut store = Store::new();

    let password = "123";
loop{
        println!(" ****NOTE: Enter 'quit' or 'exit' to logout from store...  ");
        println!();
        println!("Enter password to access store management:");
        let mut input_password = String::new();
        std::io::stdin()
            .read_line(&mut input_password)
            .expect("Failed to read line");

        if input_password.trim() == password {
            println!("ACCESS GRANTED...");

            loop {
                println!("1. Add Product");
                println!("2. Edit Product");
                println!("3. Delete Product");
                println!("4. Record Sale");
                println!("5. Record Purchase");
                println!("6. Generate Inventory Report");
                println!("7. Generate Sales Report");
                println!("8. Generate Purchases Report");
                println!("9. Exit");

                println!("Enter your choice:");
                let mut choice = String::new();
                std::io::stdin()
                    .read_line(&mut choice)
                    .expect("Failed to read line");

                let choice: i32 = choice.trim().parse().expect("Please enter a number");

                match choice {
                    1 => {
                        let mut name = String::new();
                        let mut description = String::new();
                        let mut price = String::new();
                        let mut quantity = String::new();

                        println!("Enter product name:");
                        std::io::stdin()
                            .read_line(&mut name)
                            .expect("Failed to read line");
                        let name = name.trim().to_string();

                        println!("Enter product description:");
                        std::io::stdin()
                            .read_line(&mut description)
                            .expect("Failed to read line");
                        let description = description.trim().to_string();

                        println!("Enter product price:");
                        std::io::stdin()
                            .read_line(&mut price)
                            .expect("Failed to read line");
                        let price: i32 = price.trim().parse().expect("Please enter a valid price");

                        println!("Enter product quantity:");
                        std::io::stdin()
                            .read_line(&mut quantity)
                            .expect("Failed to read line");
                        let quantity: i32 = quantity
                            .trim()
                            .parse()
                            .expect("Please enter a valid quantity");

                        if let Err(err) =
                            store.add_product(name.clone(), description.clone(), price, quantity)
                        {
                            println!("Error: {}", err);
                        }
                    }
                    2 => {
                        let mut name = String::new();
                        let mut description = String::new();
                        let mut price = String::new();
                        let mut quantity = String::new();

                        println!("Enter product name:");
                        std::io::stdin()
                            .read_line(&mut name)
                            .expect("Failed to read line");
                        let name = name.trim().to_string();

                        println!("Enter new product description:");
                        std::io::stdin()
                            .read_line(&mut description)
                            .expect("Failed to read line");
                        let description = description.trim().to_string();

                        println!("Enter new product price:");
                        std::io::stdin()
                            .read_line(&mut price)
                            .expect("Failed to read line");
                        let price: i32 = price.trim().parse().expect("Please enter a valid price");

                        println!("Enter new product quantity:");
                        std::io::stdin()
                            .read_line(&mut quantity)
                            .expect("Failed to read line");
                        let quantity: i32 = quantity
                            .trim()
                            .parse()
                            .expect("Please enter a valid quantity");

                        if let Err(err) =
                            store.edit_product(name.clone(), description.clone(), price, quantity)
                        {
                            println!("Error: {}", err);
                        }
                    }
                    3 => {
                        let mut name = String::new();

                        println!("Enter product name:");
                        std::io::stdin()
                            .read_line(&mut name)
                            .expect("Failed to read line");
                        let name = name.trim().to_string();

                        if let Err(err) = store.delete_product(name.clone()) {
                            println!("Error: {}", err);
                        }
                    }
                    4 => {
                        let mut name = String::new();
                        let mut quantity = String::new();
                        let mut price = String::new();

                        println!("Enter product name:");
                        std::io::stdin()
                            .read_line(&mut name)
                            .expect("Failed to read line");
                        let name = name.trim().to_string();

                        println!("Enter quantity sold:");
                        std::io::stdin()
                            .read_line(&mut quantity)
                            .expect("Failed to read line");
                        let quantity: i32 = quantity
                            .trim()
                            .parse()
                            .expect("Please enter a valid quantity");

                        println!("Enter sale price:");
                        std::io::stdin()
                            .read_line(&mut price)
                            .expect("Failed to read line");
                        let price: i32 = price.trim().parse().expect("Please enter a valid price");

                        if let Err(err) = store.record_sale(name.clone(), quantity, price) {
                            println!("Error: {}", err);
                        }
                    }
                    5 => {
                        let mut name = String::new();
                        let mut quantity = String::new();
                        let mut price = String::new();

                        println!("Enter product name:");
                        std::io::stdin()
                            .read_line(&mut name)
                            .expect("Failed to read line");
                        let name = name.trim().to_string();

                        println!("Enter quantity purchased:");
                        std::io::stdin()
                            .read_line(&mut quantity)
                            .expect("Failed to read line");
                        let quantity: i32 = quantity
                            .trim()
                            .parse()
                            .expect("Please enter a valid quantity");

                        println!("Enter purchase price:");
                        std::io::stdin()
                            .read_line(&mut price)
                            .expect("Failed to read line");
                        let price: i32 = price.trim().parse().expect("Please enter a valid price");

                        if let Err(err) = store.record_purchase(name.clone(), quantity, price) {
                            println!("Error: {}", err);
                        }
                    }
                    6 => {
                        println!("{}", store.generate_inventory_report());
                    }
                    7 => {
                        println!("{}", store.generate_sales_report());
                    }
                    8 => {
                        println!("{}", store.generate_purchases_report());
                    }
                    9 => {
                        println!("Exiting the store...");
                        break;
                    }
                    _ => {
                        println!("Invalid choice. Please choose a valid option.");
                    }
                }
            }
        } else if input_password.trim() == "exit" || input_password.trim() == "quit" {
            println!("Exiting the Software...");
            println!("VISIT AGAIN..");
            break;
        } else {
            println!("Incorrect password. Access denied.");
        }
            
        }
    }
