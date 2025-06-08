use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

use crate::recipe::Recipe;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct RecipeWeek {
    pub id: i32,
    pub description: Option<String>,
    pub recipes: Vec<Recipe>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
pub struct WeekDB {
    pub id: i32,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl RecipeWeek {
    pub fn new(
        id: i32,
        description: Option<String>,
        recipes: Vec<Recipe>,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            description,
            recipes,
            created_at,
        }
    }

    pub async fn get_latest(pool: &Pool<Postgres>) -> Result<Option<Self>, sqlx::Error> {
        let week = sqlx::query!("SELECT week_id AS id FROM week ORDER BY created_at DESC LIMIT 1")
            .fetch_optional(pool)
            .await?;

        if let Some(week) = week {
            let week = RecipeWeek::get_by_id(pool, week.id).await?;

            Ok(week)
        } else {
            Ok(None)
        }
    }
}

impl RecipeWeek {
    pub async fn delete_by_id(pool: &Pool<Postgres>, id: i32) -> Result<(), sqlx::Error> {
        let result = sqlx::query!("DELETE FROM week WHERE week_id = $1", id)
            .execute(pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    pub async fn get_by_id(pool: &Pool<Postgres>, id: i32) -> Result<Option<Self>, sqlx::Error> {
        let week = sqlx::query_as!(
            WeekDB,
            "SELECT week_id AS id, description, created_at AS \"created_at: DateTime<Utc>\" FROM week WHERE week_id = $1",
            id
        )
        .fetch_optional(pool)
        .await?;

        if let Some(week) = week {
            let recipes = sqlx::query!("SELECT recipe_id FROM week_recipe WHERE week_id = $1", id)
                .fetch_all(pool)
                .await?;

            let mut fetched_recipes = Vec::new();

            for recipe in &recipes {
                let recipe_id = recipe.recipe_id;
                if let Some(recipe) = Recipe::get_by_id(pool, recipe_id).await? {
                    fetched_recipes.push(recipe);
                }
            }

            Ok(Some(RecipeWeek {
                id: week.id,
                description: week.description,
                recipes: fetched_recipes,
                created_at: week.created_at,
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeWeekUnsaved {
    pub description: String,
    pub recipe_ids: Vec<i32>,
}

impl RecipeWeekUnsaved {
    pub fn new(description: String, recipe_ids: Vec<i32>) -> Self {
        Self {
            description,
            recipe_ids,
        }
    }

    pub async fn insert(&self, pool: &Pool<Postgres>) -> Result<RecipeWeek, sqlx::Error> {
        let week = sqlx::query!(
            "INSERT INTO week (description) VALUES ($1) RETURNING week_id, created_at AS \"created_at: DateTime<Utc>\"",
            self.description
        )
        .fetch_one(pool)
        .await?;

        let week_id = week.week_id;

        for recipe_id in &self.recipe_ids {
            sqlx::query!(
                "INSERT INTO week_recipe (week_id, recipe_id) VALUES ($1, $2)",
                week_id,
                recipe_id
            )
            .execute(pool)
            .await?;
        }

        let mut recipes = Vec::new();

        for recipe_id in &self.recipe_ids {
            let recipe = Recipe::get_by_id(pool, *recipe_id).await?;
            if let Some(recipe) = recipe {
                recipes.push(recipe);
            }
        }

        Ok(RecipeWeek::new(
            week_id,
            Some(self.description.clone()),
            recipes,
            week.created_at,
        ))
    }
}
