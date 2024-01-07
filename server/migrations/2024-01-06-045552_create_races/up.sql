-- Your SQL goes here
CREATE TABLE races (
    id SERIAL PRIMARY KEY,
    season INTEGER NOT NULL,
    round INTEGER NOT NULL,
    url TEXT NOT NULL,
    race_name TEXT NOT NULL,
    event_time TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (season) REFERENCES seasons (id),
    CONSTRAINT season_round UNIQUE (season, round)
)