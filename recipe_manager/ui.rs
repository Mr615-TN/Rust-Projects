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
                        self.recipe_ingredients.split(',').map(String::from).collect(),
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
                    self.recipe_servings = recipe.servings.to_string();
                    self.selected_recipe = Some(recipe.clone());
                    self.editing = true;
                }
            }
            Message::UpdateRecipe => {
                if let Some(recipe) = &.selected_recipe {
                    let servings = self.recipe_servings.parse().unwrap_or(recipe.servings);
                    self.recipe_Manager.update_Recipe(
                        recipe.id,
                        self.recipe_name.clone(),
                        self.recipe_ingredients.split(',').map(String::from).collect(),
                        self.recipe_instructions.split('\n').map(String::from).collect(),
                        servigns
                        );
                    self.editing = false;
                    self.selected_recipe = None;
                }
            }
            Message::CancelEdit => {
                self.editing = false;
                self.recipe_name.clear();
                self.recipe_ingredients.clear();
                self.recipe_instructions.clear();
                self.recipe_servings.clear();
            }
            Message::RecipeNameChanged(name) => {
                self.recipe_name = name;
            }
            Message::RecipeIngredientsChanged(ingredients) => {
                self.recipe_ingredients = ingredients;
            }
            Message::RecipeInstructionsChanged(instructions) => {
                self.recipe_instructions = instructions;
            }
            Message::RecipeServingsChanged(servings) => {
                self.recipe_servings = servings;
            }
            Message::RecipesSelected(recipe) => {
                self.selected_recipe = Some(recipe);
                self.editing =false;
            }
            Message::DeleteRecipe(id) => {
                if self.recipe_Manager.delete_Recipe(id) {
                    self.selected_recipe = None;
                } 
            }
            Message::SaveRecipes => {
                if let Err(e) = self.recipe_Manager.save_ToFile("recipes.json") {
                    self.error_message = Some(format!("Failed to save recipes: {}", e))
                }
            }
            Message::LoadRecipes => {
                match self.recipe_Manager.load_FromFile("recipes.json") {
                    Ok(_) => self.selected_recipe = None,
                    Err(e) => self.error_message = Some(format!("Failed to load recipes: {}", e))
                }
            }
        }
    }
    fn view(&self) -> Element<Message> {
        let recipe form = Column::new()
        .push(TextInput::new(
            "Enter recipe name ....",
            &self.recipe_name,
            Message::RecipeNameChanged,
        ))
        .push(TextInput::new(
            "Enter ingredients (comma separated) ....",
            &self.recipe_ingredients,
            Message::RecipeIngredientsChanged,
        ))
        .push(TextInput::new(
            "Enter instructions (line separated) ....",
            &self.recipe_instructions,
            Message::RecipeInstructionsChanged,
        ))
        .push(TextInput::new(
            "Enter servings ....",
            &self.recipe_servings,
            Message::RecipeServingsChanged,
        ))
        .push(
            if self.editing {
                Button::new(Text::new("Update Recipe")).on_press(Message::UpdateRecipe)
            }
            else {
                Button::new(Text::new("Add Recipe")).on_press(Message::AddRecipe)
            }
        )
        .push(
            if self.editing {
                Button::new(Text::new("Cancel")).on_press(Message::CancelEdit)
            }
            else {
                Button::new(Text::new(""))
            }
        );
        let recipes: Element<_> = self
        .recipe_Manager
        .get_AllRecipes()
        .iter()
        .fold(Column::new().spacing(10), |column, recipe| {
            column.push(
                Row::new()
                .push(
                    Button::new(Tex::new(&recipe.name))
                    .on_press(Message::RecipesSelected(recipe.clone()))
                )
                .push(
                    Button::new(Text::new("Edit"))
                    .on_press(Message::EditRecipe(recipe.id))
                )
                .push(
                    Button::new(Tex::new("Delete"))
                    .on_press(Message::DeleteRecipe(recipe.id))
                )
            )
        })
        .into();
        let recipes_list = Scrollable::new(recipes)
        .height(Length::Fill);

        let recipe_detail = self.selected_recipe.as_ref().map_or(
            Column::new().push(Text::new("No recipe selected")),
            |recipe| {
                
            }
        )
    }
