use sqlx::{FromRow, PgPool};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct FoodItem {
    pub fdc_id: u32,
    pub description: String,
    pub brand_owner: String,
    pub brand_name: Option<String>,
    pub gtin_upc: u64,
    pub ingredients_str: String,
    pub not_a_significant_source_of: Option<String>,
    pub serving_size: f32,
    pub serving_size_unit: String,
    pub household_serving: String,
    pub branded_food_category: Option<String>,
    pub short_description: Option<String>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Nutrient {
    pub fdc_id: u32,
    pub nutrient_id: u32,
    pub amount: f32 // per 100g
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct FoodDetails {
    pub food: FoodItem,
    pub ingredients: Vec<String>,
    pub nutrients: Vec<Nutrient>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    id: u128,
    google_id: String,
    email: String,
    full_name: String,
    given_name: Option<String>,
    family_name: Option<String>,
    avatar_url: Option<String>,
    locale: Option<String>,
    access_token: Option<String>,
    refresh_token: Option<String>,
    token_expires_at: Option<Chrono::DateTime<chrono::Utc>>,
    created_at: Chrono::DateTime<chrono::Utc>,
    updated_at: Chrono::DateTime<chrono::Utc>,
}