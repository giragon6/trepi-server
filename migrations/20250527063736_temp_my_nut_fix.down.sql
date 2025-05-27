ALTER TABLE nutrients
    ALTER COLUMN min_year_acquired TYPE INTEGER USING min_year_acquired::INTEGER;