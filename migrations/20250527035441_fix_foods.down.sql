ALTER TABLE foods
    ALTER COLUMN gtin_upc TYPE INTEGER USING gtin_upc::INTEGER;

ALTER TABLE foods 
    ADD CONSTRAINT foods_gtin_upc_key UNIQUE (gtin_upc);

ALTER TABLE foods
    ALTER COLUMN food_category_id TYPE INTEGER USING food_category_id::INTEGER;

ALTER TABLE foods
    ALTER COLUMN household_serving SET NOT NULL;