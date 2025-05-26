use sqlx::{PgPool};
use crate::{
    models::{FoodItem, Nutrient, FoodDetails},
    schema::{FilterOptions},
};

pub async fn fetch_foods(pool: &PgPool, filter_options: FilterOptions) -> Result<Vec<FoodItem>, sqlx::Error> {
    let mut query = String::from(
        "SELECT fdc_id, brand_owner, brand_name, gtin_upc, ingredient_str,
        not_a_significant_source_of, serving_size, serving_size_unit,
        household_serving, branded_food_category, short_description
        FROM food_items",
    );

    let mut conditions = Vec::new();
    // if let Some(ref data_type) = filter_options.data_type {
    //     conditions.push(format!("data_type = '{}'", data_type));
    // }
    if let Some(ref brand_owner) = filter_options.brand_owner {
        conditions.push(format!("brand_owner ILIKE '%{}%'", brand_owner));
    }
    if let Some(ref brand_name) = filter_options.brand_name {
        conditions.push(format!("brand_name ILIKE '%{}%'", brand_name));
    }
    if let Some(ref ingredient) = filter_options.ingredient {
        conditions.push(format!("ingredients_str ILIKE '%{}%'", ingredient));
    }
    if let Some(ref branded_food_category) = filter_options.branded_food_category {
        conditions.push(format!("branded_food_category ILIKE '%{}%'", branded_food_category));
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    query.push_str(&format!(" ORDER BY {fdc_id}", fdc_id = filter_options.sort_by.unwrap_or("fdc_id".to_string())));

    if let Some(sort_order) = filter_options.sort_order {
        query.push_str(&format!(" {}", sort_order.to_uppercase()));
    } else {
        query.push_str(" ASC");
    }

    if let Some(page_size) = filter_options.page_size {
        query.push_str(&format!(" LIMIT {}", page_size));
    }
    
    if let Some(page_number) = filter_options.page_number {
        let offset = (page_number - 1) * filter_options.page_size.unwrap_or(10);
        query.push_str(&format!(" OFFSET {}", offset));
    }

    sqlx::query_as::<_, FoodItem>(&query)
        .fetch_all(pool)
        .await
}

pub async fn fetch_food_details(pool: &PgPool, fdc_id: i32) -> Result<FoodDetails, sqlx::Error> {
    let food: FoodItem = sqlx::query_as!(
        FoodItem,
        r#"SELECT fdc_id, brand_owner, brand_name, gtin_upc, ingredients_str,
        not_a_significant_source_of, serving_size, serving_size_unit,
        household_serving, branded_food_category, short_description
        FROM foods WHERE fdc_id = $1"#,
        fdc_id
    )
    .fetch_one(pool)
    .await?;

    let nutrients: Vec<Nutrient> = sqlx::query_as!(
        Nutrient,
        r#"SELECT food_id, nutrient_id, amount FROM nutrients WHERE food_id = $1"#,
        fdc_id
    )
    .fetch_all(pool)
    .await?;

    Ok(FoodDetails {
        food,
        nutrients,
    })
}