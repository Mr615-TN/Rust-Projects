use iced::widget::{Button, Column, Container, Row, Scrollable, Text, TextInput};
use iced::{Element, Length, Sandbox, Settings};
use iced::theme;
use iced::Color;
use crate::recipe_manager::RecipeManager;
use crate::recipe::Recipe;

pub fn recipeManagerGUI() {
    recipe_Manager: RecipeManager,
    recipe_name: String,
    recipe_ingredients: String,
    recipe_instructions: String,
    recipe_servings: String,
    selected_recipe: Option<Recipe>,
    error_message: Option<String>,
    editing: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    AddRecipe,
    EditRecipe(u32),
    UpdateRecipe,
    CancelEdit,
    RecipeNameChanged(String),
    RecipeIngredientsChanged(String),
    RecipeInstructionsChanged(String),
    RecipeServingsChanged(String),
    RecipesSelected(Recipe),
    DeleteRecipe(u32),
    SaveRecipes,
    LoadRecipes,
}

impl Sandbox for recipeManagerGUI { 
    type Message = Message;
    fn new() -> Self {
        Self {
            recipe_Manager: RecipeManager::new(),
            recipe_name: String::new(),
            recipe_ingredients: String::new(),
            recipe_instructions: String::new(),
            recipe_servings: String::new(),
            selected_recipe: None,
            error_message: None,
            editing: false,
        }
    }
    fn title(&self) -> String {
        String::from("Recipe Manager")
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::AddRecipe => {
                if !self.recipe_name.is_empty() {
                    let servings = self.recipe_servings.parse().unwrap_or(1);
                    self.recipe_Manager.add_Recipe(
                        self.recipe_name.clone(),
                        self.recipe_ingredients.split(', ').map(String::from).collect(),
                        self.recipe_instructions.split('\n').map(String::from).collect(),
                        servings,
                        );
                    self.recipe_name.clear();
                    self.recipe_ingredients.clear();
                    self.recipe_instructions.clear();
                    self.recipe_servings.clear();
                }
            }
            Message::EditRecipe(id) => {
                if let Some(recipe) = self.recipe_Manager.get_Recipe(id) {
                    self.recipe_name = recipe.name.clone();
                    self.recipe_ingredients = recipe.ingredients.join(", ");
                    self.recipe_instructions = recipe.instructions.join("\n");
                }
            }
        }
    }
}
