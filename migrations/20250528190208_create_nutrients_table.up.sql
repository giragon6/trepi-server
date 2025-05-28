CREATE TABLE
    IF NOT EXISTS nutrients (
        food_nutrient_id INTEGER NOT NULL,
        food_id INTEGER NOT NULL,
        nutrient_id INTEGER NOT NULL,
        amount NUMERIC NOT NULL,
        data_points NUMERIC,
        derivation_id NUMERIC,
        min NUMERIC,
        max NUMERIC,
        median NUMERIC,
        loq NUMERIC,
        footnote TEXT,
        percent_daily_value NUMERIC,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        PRIMARY KEY (food_nutrient_id)
    );