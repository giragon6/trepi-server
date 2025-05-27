DROP TABLE IF EXISTS nutrients;

CREATE TABLE
    IF NOT EXISTS nutrients (
        food_id INTEGER NOT NULL,
        nutrient_id INTEGER NOT NULL,
        amount DECIMAL(10, 10) NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        CONSTRAINT fk_food
            FOREIGN KEY (food_id)
            REFERENCES foods (fdc_id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
        PRIMARY KEY (food_id, nutrient_id)
    );
