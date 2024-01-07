-- Your SQL goes here
CREATE TABLE standings (
    id SERIAL PRIMARY KEY,
    race INTEGER NOT NULL,
    driver_id TEXT NOT NULL,
    constructor_id TEXT NOT NULL,
    position INTEGER NOT NULL,
    position_text TEXT NOT NULL,
    points INTEGER NOT NULL,
    wins INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (race) REFERENCES races (id),
    FOREIGN KEY (driver_id) REFERENCES drivers (id),
    FOREIGN KEY (constructor_id) REFERENCES constructors (id),
    CONSTRAINT race_driver UNIQUE (race, driver_id)
);