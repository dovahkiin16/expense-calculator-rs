-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT now()
);