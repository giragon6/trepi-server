ALTER TABLE foods
    ALTER COLUMN brand_owner DROP NOT NULL;

ALTER TABLE foods
    ALTER COLUMN gtin_upc DROP NOT NULL;

ALTER TABLE foods
    ALTER COLUMN ingredients_str DROP NOT NULL;

ALTER TABLE foods
    ALTER COLUMN serving_size DROP NOT NULL;

ALTER TABLE foods
    ALTER COLUMN serving_size_unit DROP NOT NULL;

ALTER TABLE foods  
    ALTER COLUMN household_serving DROP NOT NULL;