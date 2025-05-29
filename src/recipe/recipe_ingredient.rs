use super::ingredient::Ingredient;

#[derive(Debug, Clone, PartialEq)]
pub struct RecipeIngredient {
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
