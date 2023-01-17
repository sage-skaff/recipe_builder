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
                println!("Enter recipe name:");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Failed to read line");

                let mut ingredients = Vec::new();
                let mut instructions = Vec::new();
                loop {
                    println!("Enter ingredient (or 'done' to finish):");
                    let mut ingredient = String::new();
                    io::stdin()
                        .read_line(&mut ingredient)
                        .expect("Failed to read line");
                    if ingredient.trim() == "done" {
                        break;
                    }
                    ingredients.push(ingredient);
                }
                loop {
                    println!("Enter instruction (or 'done' to finish):");
                    let mut instruction = String::new();
                    io::stdin()
                        .read_line(&mut instruction)
                        .expect("Failed to read line");
                    if instruction.trim() == "done" {
                        break;
                    }
                    instructions.push(instruction);
                }
                let recipe = Recipe {
                    name: name,
                    ingredients: ingredients,
                    instructions: instructions,
                };
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
            3 => {
                println!("Thanks for using Recipe Builder!");
                break;
            }
            _ => println!("Please enter a valid option"),
        }
    }
}
struct Recipe {
    name: String,
    ingredients: Vec<String>,
    instructions: Vec<String>,
}
// pub fn user_input(input) {
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read line");
// }
