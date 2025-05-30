CREATE OR REPLACE VIEW food_with_nutrients AS
SELECT
  f.fdc_id,
  f.data_type,
  f.item_description,
  f.food_category_id,
  f.brand_owner,
  f.brand_name,
  f.gtin_upc,
  f.ingredients_str,
  f.not_a_significant_source_of,
  f.serving_size,
  f.serving_size_unit,
  f.household_serving,
  f.branded_food_category,
  COALESCE(
    ARRAY_AGG(
      to_jsonb(n) 
      ORDER BY n.food_nutrient_id
    ) FILTER (WHERE n.food_nutrient_id IS NOT NULL),
    ARRAY[]::jsonb[]
  ) AS nutrients
FROM foods f
LEFT JOIN nutrients n ON n.food_id = f.fdc_id
GROUP BY
  f.fdc_id,
  f.data_type,
  f.item_description,
  f.food_category_id,
  f.brand_owner,
  f.brand_name,
  f.gtin_upc,
  f.ingredients_str,
  f.not_a_significant_source_of,
  f.serving_size,
  f.serving_size_unit,
  f.household_serving,
  f.branded_food_category;