use crate::error::Error;
use crate::http::Result;
use crate::recipe::RecipeUnsaved;
use crate::{http::ApiContext, recipe::Recipe};
use axum::routing::post;
use axum::{Extension, Json, Router, routing::get};

pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route("/api/recipes", get(get_all_recipes).post(create_recipes))
        .route("/api/recipe", post(create_recipe))
        .route(
            "/api/recipe/{recipe_id}",
            get(get_recipe_by_id).delete(delete_recipe_by_id),
        )
}

#[axum::debug_handler]
async fn get_all_recipes(ctx: Extension<ApiContext>) -> Result<Json<Vec<Recipe>>> {
    let recipes = Recipe::get_all(&ctx.db).await?;
    Ok(axum::Json(recipes))
}

async fn delete_recipe_by_id(
    ctx: Extension<ApiContext>,
    axum::extract::Path(recipe_id): axum::extract::Path<i32>,
) -> Result<Json<()>> {
    Recipe::delete_by_id(&ctx.db, recipe_id).await?;
    Ok(Json(()))
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

#[axum::debug_handler]
async fn create_recipe(
    ctx: Extension<ApiContext>,
    Json(recipe): Json<RecipeUnsaved>,
) -> Result<Json<Recipe>> {
    let recipe = recipe.insert(&ctx.db).await?;
    Ok(Json(recipe))
}

#[axum::debug_handler]
async fn create_recipes(
    ctx: Extension<ApiContext>,
    Json(recipes): Json<Vec<RecipeUnsaved>>,
) -> Result<Json<Vec<Recipe>>> {
    let mut recipes_saved = Vec::new();
    for recipe_unsaved in &recipes {
        log::debug!("Inserting recipe: {}", recipe_unsaved.name);
        let recipe = recipe_unsaved.insert(&ctx.db).await?;
        recipes_saved.push(recipe);
    }

    Ok(Json(recipes_saved))
}
