use anyhow::Context;
use axum::Router;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use vecko_meny_api::{config::Config, http};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init();

    let config = Config::parse();

    let db = PgPoolOptions::new()
        // The default connection limit for a Postgres server is 100 connections, minus 3 for superusers.
        // Since we're using the default superuser we don't have to worry about this too much,
        // although we should leave some connections available for manual access.
        //
        // If you're deploying your application with multiple replicas, then the total
        // across all replicas should not exceed the Postgres connection limit.
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;
    //sqlx::migrate!().run(&db).await?;

    http::serve(config, db).await?;

    Ok(())
}
