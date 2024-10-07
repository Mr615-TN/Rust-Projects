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