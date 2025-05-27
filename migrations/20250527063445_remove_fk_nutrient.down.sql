ALTER TABLE nutrients
    ADD CONSTRAINT fk_food 
        FOREIGN KEY (food_id) 
        REFERENCES foods (fdc_id) 
        ON DELETE CASCADE 
        ON UPDATE CASCADE;