CREATE TABLE
    IF NOT EXISTS foods (
        fdc_id  INTEGER NOT NULL,
        data_type TEXT NOT NULL,
        item_description TEXT,
        food_category_id TEXT,
        brand_owner TEXT,
        brand_name TEXT,
        gtin_upc TEXT,
        ingredients_str TEXT,
        not_a_significant_source_of TEXT,
        serving_size TEXT,
        serving_size_unit TEXT,
        household_serving TEXT,
        branded_food_category TEXT,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        PRIMARY KEY (fdc_id)
    );