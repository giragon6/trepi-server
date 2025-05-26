CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS foods (
        fdc_id  INTEGER NOT NULL,
        item_description TEXT NOT NULL,
        brand_owner TEXT NOT NULL,
        brand_name TEXT,
        gtin_upc INTEGER NOT NULL UNIQUE,
        ingredients_str TEXT NOT NULL,
        not_a_significant_source_of TEXT,
        serving_size TEXT NOT NULL,
        serving_size_unit TEXT NOT NULL,
        household_serving TEXT NOT NULL,
        branded_food_category TEXT,
        short_description TEXT,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        PRIMARY KEY (fdc_id)
    );

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

CREATE TABLE
    IF NOT EXISTS users (
        id UUID NOT NULL DEFAULT (uuid_generate_v4()),
        google_id TEXT NOT NULL UNIQUE,
        email TEXT NOT NULL UNIQUE,
        full_name TEXT NOT NULL,
        given_name TEXT,
        family_name TEXT,
        avatar_url TEXT,
        locale TEXT,
        access_token TEXT,
        refresh_token TEXT,
        token_expires_at TIMESTAMPTZ,
        last_login_at TIMESTAMPTZ,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        PRIMARY KEY (id)
    );

CREATE TABLE
    IF NOT EXISTS user_roles (
        user_id UUID NOT NULL,
        user_role TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        CONSTRAINT fk_user
            FOREIGN KEY (user_id)
            REFERENCES users (id)
            ON DELETE CASCADE
            ON UPDATE CASCADE,
        PRIMARY KEY (user_id, user_role)
    );