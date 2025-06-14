use std::string::ToString;

pub use ingredient::*;
pub use ingredient_type::*;
pub use other_ingredient::*;
pub use recipe_ingredient::*;
pub use recipe_usage::*;
use serde::{Deserialize, Serialize};

mod ingredient;
mod ingredient_type;
mod other_ingredient;
mod recipe_ingredient;
mod recipe_usage;

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct Recipe {
    #[sqlx(rename = "recipe_id")]
    pub id: i32,
    pub image_url: Option<String>,
    pub name: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub other_ingredients: Vec<OtherIngredient>,
    pub instructions: Option<String>,
}

impl Recipe {
    pub fn build(
        name: String,
        image_url: Option<String>,
        ingredients: Vec<RecipeIngredientUnsaved>,
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
    pub async fn update_fields(
        id: i32,
        image_url: Option<String>,
        name: Option<String>,
        instructions: Option<String>,
        pool: &sqlx::Pool<sqlx::Postgres>,
    ) -> Result<(), sqlx::Error> {
        //TODO: Optimize with a single query

        if let Some(image_url) = image_url {
            sqlx::query!(
                "UPDATE recipe SET image_url = $1 WHERE recipe_id = $2",
                image_url,
                id
            )
            .execute(pool)
            .await?;
        }

        if let Some(name) = name {
            sqlx::query!("UPDATE recipe SET name = $1 WHERE recipe_id = $2", name, id)
                .execute(pool)
                .await?;
        }

        if let Some(instructions) = instructions {
            sqlx::query!(
                "UPDATE recipe SET instructions = $1 WHERE recipe_id = $2",
                instructions,
                id
            )
            .execute(pool)
            .await?;
        }

        Ok(())
    }
    pub async fn delete_by_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: i32,
    ) -> Result<(), sqlx::Error> {
        let result = sqlx::query!("DELETE FROM recipe WHERE recipe_id = $1", id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
    pub async fn get_all(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<Self>, sqlx::Error> {
        let recipes = sqlx::query_as!(
            RecipeDB,
            "SELECT recipe_id, image_url, name, instructions FROM recipe"
        )
        .fetch_all(pool)
        .await?;

        let mut result = Vec::new();
        for recipe in recipes {
            let ingredients = RecipeIngredient::get_by_recipe_id(pool, recipe.recipe_id).await?;
            let other_ingredients =
                OtherIngredient::get_other_ingredients_by_recipe_id(pool, recipe.recipe_id).await?;

            result.push(Recipe {
                id: recipe.recipe_id,
                image_url: recipe.image_url,
                name: recipe.name,
                ingredients,
                other_ingredients,
                instructions: recipe.instructions,
            });
        }

        Ok(result)
    }
    pub async fn add_other_ingredient(
        &self,
        pool: &sqlx::Pool<sqlx::Postgres>,
        name: &str,
    ) -> Result<OtherIngredient, sqlx::Error> {
        let other_ingredient_id = sqlx::query!(
            "INSERT INTO recipe_other_ingredient (recipe_id, ingredient_row) VALUES ($1, $2) RETURNING recipe_other_ingredient_id",
            self.id,
            name
        )
        .fetch_one(pool)
        .await?;

        let other_ingredient_id = other_ingredient_id.recipe_other_ingredient_id;
        Ok(OtherIngredient {
            ingredient_row: name.to_string(),
            id: other_ingredient_id,
        })
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
            RecipeDB,
            "SELECT recipe_id, image_url, name, instructions FROM recipe WHERE recipe_id = $1",
            id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(recipe) = recipe {
            let ingredients = RecipeIngredient::get_by_recipe_id(pool, id).await?;
            let other_ingredients =
                OtherIngredient::get_other_ingredients_by_recipe_id(pool, id).await?;

            let recipe = Recipe {
                id: recipe.recipe_id,
                image_url: recipe.image_url,
                name: recipe.name,
                ingredients,
                other_ingredients,
                instructions: recipe.instructions,
            };
            return Ok(Some(recipe));
        }

        Ok(None)
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct RecipeDB {
    pub recipe_id: i32,
    pub image_url: Option<String>,
    pub name: String,
    pub instructions: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct RecipeUnsaved {
    pub image_url: Option<String>,
    pub name: String,
    pub ingredients: Vec<RecipeIngredientUnsaved>,
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

        for ingredient in &self.ingredients {
            ingredient.insert(pool, recipe_id).await?;
        }

        let recipe = Recipe {
            id: recipe_id,
            image_url: self.image_url.clone(),
            name: self.name.clone(),
            ingredients: self
                .ingredients
                .iter()
                .map(|i| {
                    RecipeIngredient::new(
                        Ingredient::new(i.ingredient_name.clone(), i.ingredient_type.clone()),
                        i.quantity,
                        i.unit.clone(),
                    )
                })
                .collect(),
            other_ingredients: Vec::new(),
            instructions: Some(self.instructions.clone()),
        };

        for other_ingredient in &self.other_ingredients {
            recipe.add_other_ingredient(pool, other_ingredient).await?;
        }

        Ok(recipe)
    }
}
