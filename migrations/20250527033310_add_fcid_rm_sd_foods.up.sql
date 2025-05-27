ALTER TABLE foods
  ALTER COLUMN short_description TYPE INTEGER USING short_description::INTEGER;

ALTER TABLE foods
  RENAME COLUMN short_description TO food_category_id;