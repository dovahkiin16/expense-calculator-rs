CREATE TABLE expense (
    id SERIAL PRIMARY KEY,
    amount REAL NOT NULL,
    expense_type TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL
);