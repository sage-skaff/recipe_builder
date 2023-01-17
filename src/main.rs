use std::io;

fn main() {
    let mut recipes = Vec::new();

    loop {
        println!("1. Create Recipe");
        println!("2. View Recipes");
        println!("3. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

        match choice {
            1 => {
                let recipe = create_recipe();
                recipes.push(recipe);
            }
            2 => {
                for (index, recipe) in recipes.iter().enumerate() {
                    println!("{}. {}", index + 1, recipe.name);
                }
                println!("Enter recipe number to view:");
                let mut recipe_choice = String::new();
                io::stdin()
                    .read_line(&mut recipe_choice)
                    .expect("Failed to read line");
                let recipe_choice: usize = recipe_choice
                    .trim()
                    .parse()
                    .expect("Please enter a valid number");
                let recipe = &recipes[recipe_choice - 1];
                view_recipe(recipe);
            }
            3 => {
                println!("Thanks for using Recipe Builder!");
                break;
            }
            _ => println!("Please enter a valid option"),
        }
    }
}
fn create_list(item_name: &str) -> Vec<String> {
    let mut list = Vec::new();
    loop {
        println!("Enter {} (or 'done' to finish):", item_name);
        let mut item = String::new();
        io::stdin()
            .read_line(&mut item)
            .expect("Failed to read line");
        if item.trim() == "done" {
            break;
        }
        list.push(item);
    }
    list
}
fn create_recipe() -> Recipe {
    println!("Enter recipe name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let ingredients = create_list("ingredient");
    let instructions = create_list("instruction");

    Recipe {
        name: name,
        ingredients: ingredients,
        instructions: instructions,
    }
}
fn view_recipe(recipe: &Recipe) {
    println!("{}", recipe.name);
    println!("Ingredients:");
    for ingredient in recipe.ingredients.iter() {
        println!("- {}", ingredient);
    }
    println!("Instructions:");
    for (index, instruction) in recipe.instructions.iter().enumerate() {
        println!("{}. {}", index + 1, instruction);
    }
}
struct Recipe {
    name: String,
    ingredients: Vec<String>,
    instructions: Vec<String>,
}
