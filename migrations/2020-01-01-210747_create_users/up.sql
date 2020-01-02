-- Your SQL goes here
CREATE TABLE users(
                      id         SERIAL PRIMARY KEY,
                      username   VARCHAR   NOT NULL,
                      name       VARCHAR   NOT NULL,
                      password   VARCHAR   NOT NULL,
                      created_at TIMESTAMP NOT NULL DEFAULT now()
);