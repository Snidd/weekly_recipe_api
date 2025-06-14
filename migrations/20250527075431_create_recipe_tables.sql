-- Add migration script here
CREATE TYPE ingredient_type AS ENUM ('protein', 'carbohydrate', 'other');
CREATE TABLE ingredient (
    name VARCHAR(255) PRIMARY KEY,
    type ingredient_type NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE recipe (
    recipe_id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    instructions TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE recipe_ingredient (
    recipe_id INT NOT NULL,
    ingredient_name VARCHAR(255) NOT NULL,
    quantity INT NOT NULL,
    unit VARCHAR(50),
    PRIMARY KEY (recipe_id, ingredient_name),
    FOREIGN KEY (recipe_id) REFERENCES recipe(recipe_id) ON DELETE CASCADE,
    FOREIGN KEY (ingredient_name) REFERENCES ingredient(name) ON DELETE CASCADE
);
CREATE TABLE recipe_other_ingredient (
    recipe_other_ingredient_id SERIAL PRIMARY KEY,
    recipe_id INT NOT NULL,
    ingredient_row TEXT NOT NULL,
    FOREIGN KEY (recipe_id) REFERENCES recipe(recipe_id) ON DELETE CASCADE
);
CREATE TABLE recipe_usage (
    recipe_usage_id SERIAL PRIMARY KEY,
    recipe_id INT NOT NULL,
    usage_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (recipe_id) REFERENCES recipe(recipe_id) ON DELETE CASCADE
);
CREATE TABLE week (
    week_id SERIAL PRIMARY KEY,
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE week_recipe (
    week_id INT NOT NULL,
    recipe_id INT NOT NULL,
    PRIMARY KEY (week_id, recipe_id),
    FOREIGN KEY (week_id) REFERENCES week(week_id) ON DELETE CASCADE,
    FOREIGN KEY (recipe_id) REFERENCES recipe(recipe_id) ON DELETE CASCADE
);