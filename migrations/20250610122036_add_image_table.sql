-- Add migration script here
CREATE TABLE image (
    image_id SERIAL PRIMARY KEY,
    image_content BYTEA NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);