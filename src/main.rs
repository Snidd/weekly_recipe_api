use crate::recipe::Ingredient;
use recipe::Recipe;
use sqlx::postgres::PgPoolOptions;
use std::env;

//static MIGRATOR: Migrator = sqlx::migrate!(); // defaults to "./migrations"

mod recipe;

//#[async_std::main] // Requires the `attributes` feature of `async-std`
#[tokio::main]
// or #[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("env variable DATABASE_URL must be set");
    // Create a connection pool
    //  for MySQL/MariaDB, use MySqlPoolOptions::new()
    //  for SQLite, use SqlitePoolOptions::new()
    //  etc.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await?;

    let recipe = Recipe::get_by_id(&pool, 1).await?;
    if let Some(recipe) = recipe {
        println!("Recipe found: {:?}", recipe);
    } else {
        println!("No recipe found with the given ID.");
    }

    /*
    let new_recipe = Recipe::new(
        "testnamn".to_string(),
        Some("testimageurl".to_string()),
        vec![
            recipe::RecipeIngredient::new(
                Ingredient::new("kyckling".to_string(), recipe::IngredientType::Protein),
                1,
                "kg".to_string(),
            ),
            recipe::RecipeIngredient::new(
                Ingredient::new("ris".to_string(), recipe::IngredientType::Carbohydrate),
                2,
                "dl".to_string(),
            ),
        ],
        vec![String::from("2 påsar sallad"), String::from("3 ägg")],
        String::from("instructions"),
    );

    let recipe = new_recipe.insert(&pool).await?;
    println!("Inserted recipe: {:?}", recipe);*/

    //let kyckling = Ingredient::get_by_name(&pool, "kyckling").await?;

    //Ingredient::delete_by_name(&pool, "Chicken Breast").await?;

    /*
    let ingredient = crate::recipe::Ingredient::new(
        "kyckling".to_string(),
        crate::recipe::IngredientType::Protein,
    );

    ingredient.insert(&pool).await?;*/

    Ok(())
}
