-- Your SQL goes here
CREATE TABLE expenses (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    amount REAL NOT NULL,
    expense_type VARCHAR NOT NULL,
    need BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
)