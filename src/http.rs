use std::sync::Arc;

use crate::{config::Config, error::Error};

use anyhow::Context;
use axum::Router;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{add_extension::AddExtensionLayer, trace::TraceLayer};

pub mod image;
pub mod ingredients;
pub mod recipe;
pub mod week;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let app = api_router().layer(
        ServiceBuilder::new()
            // The other reason for using a single object is because `AddExtensionLayer::new()` is
            // rather verbose compared to Actix-web's `Data::new()`.
            //
            // It seems very logically named, but that makes it a bit annoying to type over and over.
            .layer(AddExtensionLayer::new(ApiContext {
                config: Arc::new(config),
                db,
            }))
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(TraceLayer::new_for_http()),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .context("error binding to port 8080")?;

    axum::serve(listener, app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router {
    // This is the order that the modules were authored in.
    recipe::router()
        .merge(ingredients::router())
        .merge(week::router())
        .merge(image::router())
    //.merge(articles::router())
}
