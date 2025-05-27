use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "ingredient_type", rename_all = "lowercase")]
pub enum IngredientType {
    Protein,
    Carbohydrate,
    Other,
}
