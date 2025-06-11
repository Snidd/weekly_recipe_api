use crate::error::Error;
use crate::http::ApiContext;
use crate::http::Result;
use axum::extract::DefaultBodyLimit;
use axum::extract::Multipart;
use axum::http::HeaderMap;
use axum::routing::post;
use axum::{Extension, Json, Router, routing::get};
use serde::Serialize;

pub fn router() -> Router {
    // By having each module responsible for setting up its own routing,
    // it makes the root module a lot cleaner.
    Router::new()
        .route("/api/image", post(create_image))
        .route(
            "/api/image/{image_id}",
            get(get_image_by_id).delete(delete_image_by_id),
        )
        .layer(DefaultBodyLimit::max(50 * 1000 * 1000))
}

#[axum::debug_handler]
async fn create_image(
    ctx: Extension<ApiContext>,
    mut multipart: Multipart,
) -> Result<Json<ImageResult>> {
    while let Some(field) = multipart.next_field().await? {
        match field.name() {
            Some(name) if name == "image" => {
                // Process the image field
                log::debug!("Processing image field");
                let data = field.bytes().await?;
                let image_id = sqlx::query_scalar!(
                    "INSERT INTO image (image_content) VALUES ($1) RETURNING image_id",
                    data.as_ref()
                )
                .fetch_one(&ctx.db)
                .await?;
                log::debug!("Inserted image with ID: {}", image_id);
                return Ok(Json(ImageResult { image_id }));
            }
            _ => {
                log::trace!("Unexpected field: {:?}", field.name());
            }
        }
    }
    Err(Error::Forbidden)
}

#[derive(Serialize)]
struct ResultOK {
    success: bool,
    message: String,
}

async fn delete_image_by_id(
    ctx: Extension<ApiContext>,
    axum::extract::Path(image_id): axum::extract::Path<i32>,
) -> Result<Json<ResultOK>> {
    sqlx::query!("DELETE FROM image WHERE image_id = $1", image_id)
        .execute(&ctx.db)
        .await?;
    log::debug!("Deleted image with ID: {}", image_id);
    Ok(Json(ResultOK {
        success: true,
        message: format!("Deleted image with ID: {}", image_id),
    }))
}

async fn get_image_by_id(
    ctx: Extension<ApiContext>,
    axum::extract::Path(image_id): axum::extract::Path<i32>,
) -> Result<(HeaderMap, Vec<u8>)> {
    let image = sqlx::query_as!(
        Image,
        "SELECT image_id, image_content FROM image WHERE image_id = $1",
        image_id
    )
    .fetch_one(&ctx.db)
    .await?;

    let mut headers = HeaderMap::new();
    headers.insert(
        "content-type",
        "image/jpeg".parse().expect("Expected valid MIME type"),
    );

    log::debug!("Retrieved image with ID: {}", image.image_id);
    Ok((headers, image.image_content))
}

#[derive(Serialize)]
pub struct ImageResult {
    pub image_id: i32,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct Image {
    image_id: i32,
    image_content: Vec<u8>,
}
