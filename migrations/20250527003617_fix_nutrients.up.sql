ALTER TABLE nutrients
    RENAME TO nutrients_old;

CREATE TABLE
    IF NOT EXISTS nutrients (
        food_nutrient_id INTEGER NOT NULL,
        food_id INTEGER NOT NULL,
        nutrient_id INTEGER NOT NULL,
        amount NUMERIC NOT NULL,
        data_points INTEGER,
        derivation_id INTEGER,
        min NUMERIC,
        max NUMERIC,
        median NUMERIC,
        loq NUMERIC,
        footnote TEXT,
        min_year_acquired INTEGER,
        percent_daily_value NUMERIC,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        CONSTRAINT fk_food
            FOREIGN KEY (food_id)
            REFERENCES foods (fdc_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
        PRIMARY KEY (food_nutrient_id)
    );

DROP TABLE IF EXISTS nutrients_old;