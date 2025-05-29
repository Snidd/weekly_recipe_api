use serde::Serialize;
use sqlx::{Pool, Postgres};

use super::ingredient_type::IngredientType;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow, Serialize)]
pub struct Ingredient {
    pub name: String,
    pub ingredient_type: IngredientType,
}

impl Ingredient {
    pub fn new(name: String, ingredient_type: IngredientType) -> Self {
        let lower_name = name.to_lowercase();
        Self {
            name: lower_name,
            ingredient_type,
        }
    }
    pub async fn get_all(pool: &Pool<Postgres>) -> Result<Vec<Self>, sqlx::Error> {
        let ingredients = sqlx::query_as!(
            Ingredient,
            "SELECT name, type as \"ingredient_type: IngredientType\" FROM ingredient"
        )
        .fetch_all(pool)
        .await?;

        Ok(ingredients)
    }
    pub async fn delete_by_name(pool: &Pool<Postgres>, name: &str) -> Result<(), sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM ingredient WHERE name = $1",
            name.to_lowercase()
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
    pub async fn get_by_name(
        pool: &Pool<Postgres>,
        name: &str,
    ) -> Result<Option<Self>, sqlx::Error> {
        let ingredient = sqlx::query_as!(
            Ingredient,
            "SELECT name, type as \"ingredient_type: IngredientType\" FROM ingredient WHERE name = $1",
            name.to_lowercase()
        )
        .fetch_optional(pool)
        .await?;

        Ok(ingredient)
    }
    pub async fn insert(&self, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        let ingredient = sqlx::query!(
            "INSERT INTO ingredient (name, type) VALUES ($1, $2)",
            self.name,
            self.ingredient_type as _
        );
        let result = ingredient.execute(pool).await;

        if result.is_ok() {
            return Ok(());
        };

        let err = result.err().unwrap();
        match err {
            sqlx::Error::Database(e) if e.is_unique_violation() => return Ok(()),
            _ => return Err(err),
        }
    }
}
