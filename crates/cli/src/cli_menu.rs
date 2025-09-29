use core::inventory::Inventory;
use std::io;

// Helper function to get trimmed user input
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn main_menu() {
    // Inventory::new() now returns a Result, so we handle it.
    let inventory = Inventory::new().expect("Failed to initialize database. Make sure the program has permissions to create inventory.db");

    loop {
        println!("
===== INVENTORY MENU =====");
        println!("1. List Ingredients");
        println!("2. Add Ingredient");
        println!("3. Edit Ingredient");
        println!("4. Delete Ingredient");
        println!("5. Exit");
        
        let choice = get_input("Enter your choice:");

        match choice.as_str() {
            "1" => list_ingredients(&inventory),
            "2" => add_ingredient(&inventory),
            "3" => edit_ingredient(&inventory),
            "4" => delete_ingredient(&inventory),
            "5" => {
                println!("Exiting application.");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn list_ingredients(inventory: &Inventory) {
    println!("
--- Current Inventory ---");
    match inventory.list_ingredients() {
        Ok(ingredients) => {
            if ingredients.is_empty() {
                println!("No ingredients found.");
            } else {
                for ingredient in ingredients {
                    println!("- {}: {}", ingredient.name, ingredient.quantity);
                }
            }
        }
        Err(e) => println!("Error fetching ingredients: {}", e),
    }
    println!("-------------------------");
}

fn add_ingredient(inventory: &Inventory) {
    println!("
--- Add New Ingredient ---");
    let name = get_input("Enter ingredient name:");
    
    let quantity: u32 = loop {
        let qty_str = get_input("Enter quantity:");
        match qty_str.parse() {
            Ok(num) => break num,
            Err(_) => println!("Invalid quantity. Please enter a number."),
        };
    };

    match inventory.add_ingredient(name.clone(), quantity) {
        Ok(_) => println!("Successfully added/updated {}.", name),
        Err(e) => println!("Error adding ingredient: {}", e),
    }
}

fn edit_ingredient(inventory: &Inventory) {
    println!("
--- Edit Ingredient Quantity ---");
    let name = get_input("Enter the name of the ingredient to edit:");

    let new_quantity: u32 = loop {
        let qty_str = get_input("Enter the new quantity:");
        match qty_str.parse() {
            Ok(num) => break num,
            Err(_) => println!("Invalid quantity. Please enter a number."),
        };
    };

    match inventory.update_ingredient(name.clone(), new_quantity) {
        Ok(_) => println!("Successfully updated {}.", name),
        Err(e) => println!("Error updating ingredient: {}", e),
    }
}

fn delete_ingredient(inventory: &Inventory) {
    println!("
--- Delete Ingredient ---");
    let name = get_input("Enter the name of the ingredient to delete:");

    match inventory.delete_ingredient(name.clone()) {
        Ok(_) => println!("Successfully deleted {}.", name),
        Err(e) => println!("Error deleting ingredient: {}", e),
    }
}
