use super::ingredient_type::IngredientType;

#[derive(Debug, Clone)]
pub struct Ingredient {
    pub name: String,
    pub quantity: String,
    pub ingredient_type: IngredientType,
}
