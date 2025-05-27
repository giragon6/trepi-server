ALTER TABLE foods
    ALTER COLUMN gtin_upc TYPE TEXT;

ALTER TABLE foods
    DROP CONSTRAINT foods_gtin_upc_key;

ALTER TABLE foods
    ALTER COLUMN food_category_id TYPE TEXT;

ALTER TABLE foods
    ALTER COLUMN household_serving DROP NOT NULL;