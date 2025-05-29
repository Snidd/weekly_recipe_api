use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow, Serialize)]
pub struct OtherIngredient {
    pub ingredient_row: String,
    #[sqlx(rename = "recipe_other_ingredient_id")]
    pub id: i32,
}

impl OtherIngredient {
    pub async fn delete_by_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        id: i32,
    ) -> Result<(), sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM recipe_other_ingredient WHERE recipe_other_ingredient_id = $1",
            id
        )
        .execute(pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
    pub async fn get_other_ingredients_by_recipe_id(
        pool: &sqlx::Pool<sqlx::Postgres>,
        recipe_id: i32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        let ingredients = sqlx::query_as!(
            OtherIngredient,
            "SELECT ingredient_row, recipe_other_ingredient_id as id FROM recipe_other_ingredient WHERE recipe_id = $1",
            recipe_id
        )
        .fetch_all(pool)
        .await?;

        Ok(ingredients)
    }
}
