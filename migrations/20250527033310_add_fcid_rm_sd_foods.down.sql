ALTER TABLE foods
    ALTER COLUMN food_category_id TEXT;

ALTER TABLE foods
    ALTER COLUMN food_category_id RENAME TO short_description;