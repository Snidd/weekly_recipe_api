-- Add migration script here
ALTER TABLE recipe
ADD COLUMN image_url VARCHAR(255);