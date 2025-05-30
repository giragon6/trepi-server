use sqlx::{FromRow};
use sqlx::types::BigDecimal;
use serde::{Deserialize, Serialize};
use chrono;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct FoodItem {
    pub fdc_id: i32,
    pub data_type: String,
    pub item_description: Option<String>,
    pub food_category_id: Option<String>,
    pub brand_owner: Option<String>,
    pub brand_name: Option<String>,
    pub gtin_upc: Option<String>,
    pub ingredients_str: Option<String>,
    pub not_a_significant_source_of: Option<String>,
    pub serving_size: Option<String>,
    pub serving_size_unit: Option<String>,
    pub household_serving: Option<String>,
    pub branded_food_category: Option<String>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Nutrient {
    pub food_nutrient_id: i32,
    pub food_id: i32,
    pub nutrient_id: i32,
    pub amount: BigDecimal, // per 100g
    pub data_points: Option<BigDecimal>,
    pub derivation_id: Option<BigDecimal>,
    pub min: Option<BigDecimal>, // per 100g
    pub max: Option<BigDecimal>, // per 100g
    pub median: Option<BigDecimal>, // per 100g
    pub loq: Option<BigDecimal>, // Limit of Quantitation, per 100g
    pub footnote: Option<String>,
    pub percent_daily_value: Option<BigDecimal>, // per 100g
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct FoodDetails {
    pub fdc_id: Option<i32>,
    pub data_type: Option<String>,
    pub item_description: Option<String>,
    pub food_category_id: Option<String>,
    pub brand_owner: Option<String>,
    pub brand_name: Option<String>,
    pub gtin_upc: Option<String>,
    pub ingredients_str: Option<String>,
    pub not_a_significant_source_of: Option<String>,
    pub serving_size: Option<String>,
    pub serving_size_unit: Option<String>,
    pub household_serving: Option<String>,
    pub branded_food_category: Option<String>,
    pub nutrients: Option<Vec<sqlx::types::JsonValue>>,
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
    token_expires_at: Option<chrono::DateTime<chrono::Utc>>,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}