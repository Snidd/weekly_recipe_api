use serde::Serialize;

use super::{IngredientType, ingredient::Ingredient};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RecipeIngredient {
    #[serde(flatten)]
    pub ingredient: Ingredient,
    pub quantity: i32,
    pub unit: String,
}

impl RecipeIngredient {
    pub fn new(ingredient: Ingredient, quantity: i32, unit: String) -> Self {
        Self {
            ingredient,
            quantity,
            unit,
        }
    }

    pub async fn get_by_recipe_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        recipe_id: i32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let ingredients = sqlx::query_as!(
            RecipeIngredientDB,
            "select name, quantity, unit, type as  \"ingredient_type: IngredientType\"
            from recipe_ingredient
            inner join ingredient on name = ingredient_name
            where recipe_id = $1",
            recipe_id
        )
        .fetch_all(pool)
        .await?;

        let ingredients = ingredients
            .into_iter()
            .map(|db_ingredient| RecipeIngredient {
                ingredient: Ingredient::new(db_ingredient.name, db_ingredient.ingredient_type),
                quantity: db_ingredient.quantity,
                unit: db_ingredient.unit.unwrap_or_default(),
            })
            .collect();

        Ok(ingredients)
    }
    pub async fn insert(
        &self,
        pool: &sqlx::Pool<sqlx::Postgres>,
        recipe_id: i32,
    ) -> Result<(), sqlx::Error> {
        self.ingredient.insert(pool).await?;

        sqlx::query!(
            "INSERT INTO recipe_ingredient (recipe_id, ingredient_name, quantity, unit) VALUES ($1, $2, $3, $4)",
            recipe_id,
            self.ingredient.name,
            self.quantity,
            self.unit
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

struct RecipeIngredientDB {
    pub name: String,
    pub ingredient_type: IngredientType,
    pub quantity: i32,
    pub unit: Option<String>,
}
