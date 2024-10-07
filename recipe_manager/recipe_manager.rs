use crate::recipe::Recipe;
use std::fs;

pub struct RecipeManager {
    recipes: Vec<Recipe>,
    next_id: u32,
}

impl RecipeManager {
    pub fn new() -> Self {
        RecipeManager {
            recipes: Vec::new(),
            next_id: 1,
        }
    }
    pub fn addRecipe(&mut self, name: String, ingredients: Vec<String>, instructions: Vec<String>, servings: u32) -> u32 {
        let id = self.next_id;
        self.recipes.push(Recipe::new(id, name, ingredients, instructions, servings));
        self.next_id += 1;
        id
    }
    pub fn getAllRecipes(&self) -> &Vec<Recipe> {
        &self.recipes
    }
    pub fn getRecipe(&self, id:u32) -> Option<&Recipe> {
        self.recipes.iter().find(|r| r.id == id)
    }
}