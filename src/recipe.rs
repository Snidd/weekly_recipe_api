use std::string::ToString;

use ingredient::*;
use ingredient_type::*;
mod ingredient;
mod ingredient_type;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Recipe {
    pub id: u32,
    pub image_url: Option<String>,
    pub name: String,
    pub typed_ingredients: Vec<Ingredient>,
    pub other_ingredients: Vec<String>,
    pub instructions: String,
}

impl Recipe {
    pub fn new(
        id: u32,
        image_url: Option<String>,
        name: String,
        typed_ingredients: Vec<Ingredient>,
        other_ingredients: Vec<String>,
        instructions: String,
    ) -> Self {
        Self {
            id,
            image_url,
            name,
            typed_ingredients,
            other_ingredients,
            instructions,
        }
    }
    pub fn example_one() -> Self {
        Self::new(
            1,
            None,
            "Example Recipe".to_string(),
            vec![
                Ingredient {
                    name: "Kyckling".to_string(),
                    quantity: "500g".to_string(),
                    ingredient_type: IngredientType::Protein,
                },
                Ingredient {
                    name: "Ris".to_string(),
                    quantity: "200g".to_string(),
                    ingredient_type: IngredientType::Carbohydrate,
                },
            ],
            vec!["Salt".to_string(), "Pepper".to_string()],
            "Cook the chicken and rice together.".to_string(),
        )
    }
}
