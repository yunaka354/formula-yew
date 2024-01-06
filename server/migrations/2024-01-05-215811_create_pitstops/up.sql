-- Your SQL goes here
CREATE TABLE seasons (
    id SERIAL PRIMARY KEY,
    season INTEGER NOT NULL UNIQUE,
    url TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
)