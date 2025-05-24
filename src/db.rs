use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow)]
pub struct FoodItem {
    pub fdc_id: i32,
    pub brand_owner: String
    pub brand_name: String,
    pub gtin_upc: i32,
    pub ingredients_str: String,
    pub not_a_significant_source_of: String,
    pub serving_size: f32,
    pub serving_size_unit: String,
    pub household_serving: String,
    pub branded_food_category: String,
    pub short_description: String,
}

#[derive(Debug, FromRow)]
pub struct Nutrient {
    pub fdc_id: i32,
    pub nutrient_id: i32,
    pub amount: f32 // per 100g
}

#[derive(Debug, FromRow)]
pub struct FoodDetails {
    pub food: FoodItem,
    pub ingredients: Vec<String>,
    pub nutrients: Vec<Nutrient>,
}

pub async fn fetch_food_details(pool: &PgPool, fdc_id: i32) -> Result<FoodDetails> {
    let food: FoodItem = sqlx::query_as!(
        FoodItem,
        r#"SELECT fdc_id, brand_owner, brand_name, gtin_upc, ingredients,
        not_a_significant_source_of, serving_size, serving_size_unit,
        household_serving, branded_food_category, short_description
        FROM food_items WHERE fdc_id = $1"#,
        fdc_id
    )
    .fetch_one(pool)
    .await?;

    let ingredients: Vec<String> = food.ingredients_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let nutrients: Vec<Nutrient> = sqlx::query_as!(
        Nutrient,
        r#"SELECT fdc_id, nutrient_id, amount FROM food_nutrients WHERE fdc_id = $1"#,
        fdc_id
    )
    .fetch_all(pool)
    .await?;

    Ok(FoodDetails {
        food,
        ingredients,
        nutrients,
    })
}