ALTER TABLE foods
    ALTER COLUMN brand_owner NOT NULL;

ALTER TABLE foods
    ALTER COLUMN gtin_upc NOT NULL;

ALTER TABLE foods
    ALTER COLUMN ingredients_str NOT NULL;

ALTER TABLE foods
    ALTER COLUMN serving_size NOT NULL;

ALTER TABLE foods
    ALTER COLUMN serving_size_unit NOT NULL;

ALTER TABLE foods
    ALTER COLUMN household_serving NOT NULL;