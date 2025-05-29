use serde::{Deserialize, Serialize};
//#[derive(Debug, Clone, )]
#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "ingredient_type", rename_all = "lowercase")]
pub enum IngredientType {
    Protein,
    Carbohydrate,
    Other,
}
