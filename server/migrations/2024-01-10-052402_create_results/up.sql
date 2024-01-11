-- Your SQL goes here

CREATE TABLE race_results (
    id SERIAL PRIMARY KEY,
    race_id INTEGER NOT NULL,
    driver_id TEXT NOT NULL,
    constructor_id TEXT NOT NULL,
    position INTEGER NOT NULL,
    position_text TEXT NOT NULL,
    grid INTEGER NOT NULL,
    laps INTEGER NOT NULL,
    status TEXT NOT NULL,
    points DECIMAL(3, 1) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (race_id) REFERENCES races (id),
    FOREIGN KEY (driver_id) REFERENCES drivers (id),
    FOREIGN KEY (constructor_id) REFERENCES constructors (id),
    CONSTRAINT race_driver_for_results UNIQUE (race_id, driver_id)
);
