use crate::{
    models::{FoodDetails, FoodItem, Nutrient}, schema::FilterOptions
};
use actix_web::{get, web, HttpResponse, Result as ActixResult};
use serde_json::json;
use sqlx::PgPool;
use log::error;

#[get("/health")]
pub async fn health_check() -> ActixResult<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().json(json!({ "status": "ok" })))
}

#[get("/foods")]
pub async fn food_list_handler(
    pool: web::Data<PgPool>,
    filter_options: web::Query<FilterOptions>,
) -> ActixResult<HttpResponse, actix_web::Error> {
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

    query.push_str(&format!(" ORDER BY {fdc_id}", fdc_id = filter_options.sort_by.as_deref().unwrap_or("fdc_id")));

    if let Some(sort_order) = filter_options.sort_order.as_deref() {
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

    let foods_result = sqlx::query_as::<_, FoodItem>(&query)
        .fetch_all(pool.get_ref())
        .await;

    let foods = match foods_result {
        Ok(f) if !f.is_empty() => f,
        Ok(_) => {
            return Ok(HttpResponse::NotFound().json(json!({"status":"error","message":"No foods found"})))
        }
        Err(e) => {
            error!("DB error: {:?}", e);
            return Ok(HttpResponse::InternalServerError().json(json!({"status":"error","message":"DB error"})))
        }
    };

    // 3) success path
    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "results": foods.len(),
        "foods": foods
    })))
}

#[get("/foods/{fdc_id}")]
pub async fn food_request_handler(
    pool: web::Data<PgPool>,
    fdc_id: web::Path<i32>,
) -> ActixResult<HttpResponse, actix_web::Error> {
    let fdc_id = fdc_id.into_inner();

    let food_details_result = sqlx::query_as!(
        FoodDetails,
        r#"SELECT 
            fdc_id, 
            data_type, 
            item_description, 
            food_category_id, 
            brand_owner,
            brand_name,
            gtin_upc,
            ingredients_str,
            not_a_significant_source_of,
            serving_size,
            serving_size_unit,
            household_serving,
            branded_food_category,
            nutrients
         FROM food_with_nutrients 
         WHERE fdc_id = $1"#,
         fdc_id
    )
    .fetch_one(pool.get_ref())
    .await;

    match food_details_result {
        Ok(food_details) => {
            Ok(HttpResponse::Ok().json(json!({
                "status": "success",
                "food": food_details
            })))
        }
        Err(sqlx::Error::RowNotFound) => {
            Ok(HttpResponse::NotFound().json(json!({"status":"error","message":"Food not found"})))
        }
        Err(e) => {
            error!("DB error: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({"status":"error","message":"DB error"})))
        }
    }
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api/v1")
        .service(health_check)
        .service(food_list_handler)
        .service(food_request_handler);

    cfg.service(scope);
}