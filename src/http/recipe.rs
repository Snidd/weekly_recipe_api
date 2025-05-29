use crate::error::Error;
use crate::http::Result;
use crate::{http::ApiContext, recipe::Recipe};
use axum::{Extension, Json, Router, routing::get};

pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route("/api/recipes", get(get_all_recipes))
        .route("/api/recipe/{recipe_id}", get(get_recipe_by_id))
}

#[axum::debug_handler]
async fn get_all_recipes(ctx: Extension<ApiContext>) -> Result<Json<Vec<Recipe>>> {
    let recipes = Recipe::get_all(&ctx.db).await?;
    Ok(axum::Json(recipes))
}

async fn get_recipe_by_id(
    ctx: Extension<ApiContext>,
    axum::extract::Path(recipe_id): axum::extract::Path<i32>,
) -> Result<Json<Recipe>> {
    let recipe = Recipe::get_by_id(&ctx.db, recipe_id).await?;
    match recipe {
        Some(recipe) => Ok(Json(recipe)),
        None => Err(Error::NotFound),
    }
}
