use std::string::ToString;

pub use ingredient::*;
pub use ingredient_type::*;
pub use recipe_ingredient::*;
pub use recipe_usage::*;
use sqlx::pool;
use time::UtcDateTime;
mod ingredient;
mod ingredient_type;
mod recipe_ingredient;
mod recipe_usage;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Recipe {
    #[sqlx(rename = "recipe_id")]
    pub id: i32,
    pub image_url: Option<String>,
    pub name: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub other_ingredients: Vec<String>,
    pub instructions: Option<String>,
    pub usage: Vec<UtcDateTime>,
}

impl Recipe {
    pub fn new(
        name: String,
        image_url: Option<String>,
        ingredients: Vec<RecipeIngredient>,
        other_ingredients: Vec<String>,
        instructions: String,
    ) -> RecipeUnsaved {
        RecipeUnsaved {
            image_url,
            name,
            ingredients,
            other_ingredients,
            instructions,
        }
    }
    pub async fn add_other_ingredient(
        &self,
        pool: &sqlx::Pool<sqlx::Postgres>,
        name: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO recipe_other_ingredient (recipe_id, ingredient_row) VALUES ($1, $2)",
            self.id,
            name
        )
        .execute(pool)
        .await?;
        Ok(())
    }
    pub async fn get_by_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: i32,
    ) -> Result<Option<Self>, sqlx::Error> {
        /*
            SELECT
            recipe.id,
            name,
            ARRAY_AGG(roi.ingredient_row) as other_ingredients,
            JSON_AGG(json_build_array(ri.ingredient_name, ri.quantity, ri.unit))
        FROM recipe
        JOIN recipe_other_ingredient roi ON recipe.id = roi.recipe_id
        JOIN recipe_ingredient ri ON recipe.id = ri.recipe_id
        GROUP BY recipe.id
             */
        let recipe = sqlx::query_as!(
            Recipe,
            "SELECT recipe_id, image_url, name, instructions FROM recipe WHERE recipe_id = $1",
            id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(mut recipe) = recipe {
            recipe.ingredients = RecipeIngredient::get_by_recipe_id(pool, id).await?;
            recipe.other_ingredients =
                RecipeIngredient::get_other_ingredients_by_recipe_id(pool, id).await?;
            return Ok(Some(recipe));
        }

        Ok(None)
    }
}

#[derive(Debug, Clone)]

pub struct RecipeUnsaved {
    pub image_url: Option<String>,
    pub name: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub other_ingredients: Vec<String>,
    pub instructions: String,
}

impl RecipeUnsaved {
    pub async fn insert(&self, pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Recipe, sqlx::Error> {
        let recipe = sqlx::query!(
            "INSERT INTO recipe (image_url, name, instructions) VALUES ($1, $2, $3) RETURNING recipe_id",
            self.image_url,
            self.name,
            self.instructions
        )
        .fetch_one(pool)
        .await?;

        let recipe_id = recipe.recipe_id;

        let recipe = Recipe {
            id: recipe_id,
            image_url: self.image_url.clone(),
            name: self.name.clone(),
            ingredients: self.ingredients.clone(),
            other_ingredients: self.other_ingredients.clone(),
            instructions: Some(self.instructions.clone()),
            usage: vec![],
        };

        for ingredient in &self.ingredients {
            ingredient.insert(pool, recipe_id).await?;
        }

        for other_ingredient in &self.other_ingredients {
            recipe.add_other_ingredient(pool, &other_ingredient).await?;
        }

        Ok(recipe)
    }
}
