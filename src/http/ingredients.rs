use crate::http::ApiContext;
use crate::http::Result;
use crate::recipe::Ingredient;
use axum::{Extension, Json, Router, routing::get};

pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route("/api/ingredients", get(get_all_ingredients))
        .route(
            "/api/ingredient/{ingredient_name}",
            get(get_ingredient_by_name).delete(delete_ingredient_by_name),
        )
}

#[axum::debug_handler]
async fn get_all_ingredients(ctx: Extension<ApiContext>) -> Result<Json<Vec<Ingredient>>> {
    let ingredients = Ingredient::get_all(&ctx.db).await?;
    log::debug!("Retrieved {} ingredients", ingredients.len());
    Ok(axum::Json(ingredients))
}

async fn delete_ingredient_by_name(
    ctx: Extension<ApiContext>,
    axum::extract::Path(ingredient_name): axum::extract::Path<String>,
) -> Result<Json<()>> {
    log::debug!("Deleting ingredient: {}", ingredient_name);
    Ingredient::delete_by_name(&ctx.db, &ingredient_name).await?;
    Ok(Json(()))
}

async fn get_ingredient_by_name(
    ctx: Extension<ApiContext>,
    axum::extract::Path(ingredient_name): axum::extract::Path<String>,
) -> Result<Json<Ingredient>> {
    log::debug!("Retrieving ingredient: {}", ingredient_name);
    let ingredient = Ingredient::get_by_name(&ctx.db, &ingredient_name).await?;
    match ingredient {
        Some(ingredient) => Ok(Json(ingredient)),
        None => Err(crate::error::Error::NotFound),
    }
}
