use crate::http::ApiContext;
use crate::http::Result;
use crate::week::RecipeWeek;
use crate::week::RecipeWeekUnsaved;
use axum::routing::post;
use axum::{Extension, Json, Router, routing::get};

pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route("/api/week", post(create_week))
        .route(
            "/api/week/{week_id}",
            get(get_week_by_id).delete(delete_week_by_id),
        )
        .route("/api/week/latest", get(get_latest_week))
}

async fn get_latest_week(ctx: Extension<ApiContext>) -> Result<Json<RecipeWeek>> {
    let recipe_week = RecipeWeek::get_latest(&ctx.db).await?;
    match recipe_week {
        Some(recipe_week) => Ok(Json(recipe_week)),
        None => Err(crate::error::Error::NotFound),
    }
}

async fn create_week(
    ctx: Extension<ApiContext>,
    Json(recipe_week): Json<RecipeWeekUnsaved>,
) -> Result<Json<RecipeWeek>> {
    let recipe_week = recipe_week.insert(&ctx.db).await?;
    Ok(Json(recipe_week))
}

async fn get_week_by_id(
    ctx: Extension<ApiContext>,
    axum::extract::Path(week_id): axum::extract::Path<i32>,
) -> Result<Json<RecipeWeek>> {
    let recipe_week = RecipeWeek::get_by_id(&ctx.db, week_id).await?;
    match recipe_week {
        Some(recipe_week) => Ok(Json(recipe_week)),
        None => Err(crate::error::Error::NotFound),
    }
}

async fn delete_week_by_id(
    ctx: Extension<ApiContext>,
    axum::extract::Path(week_id): axum::extract::Path<i32>,
) -> Result<Json<()>> {
    RecipeWeek::delete_by_id(&ctx.db, week_id).await?;
    Ok(Json(()))
}
