use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct FilterOptions {
    pub data_type: Option<String>,
    pub page_size: Option<i32>, 
    pub page_number: Option<i32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub brand_owner: Option<String>,
    pub brand_name: Option<String>,
    pub ingredient: Option<String>,
    pub branded_food_category: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchParamOptions {
    pub query: String,
    pub data_type: Option<String>,
    pub page_size: Option<i32>,
    pub page_number: Option<i32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub brand_owner: Option<String>,
}