-- Your SQL goes here
CREATE TABLE laptimes (
    id SERIAL PRIMARY KEY,
    race_id INTEGER NOT NULL,
    driver_id TEXT NOT NULL,
    lap_number INTEGER NOT NULL,
    lap_time TEXT NOT NULL,
    position INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (race_id) REFERENCES races (id),
    FOREIGN KEY (driver_id) REFERENCES drivers (id),
    CONSTRAINT race_driver_lap_number UNIQUE (race_id, driver_id, lap_number)
);

CREATE TABLE pitstops (
    id SERIAL PRIMARY KEY,
    race_id INTEGER NOT NULL,
    driver_id TEXT NOT NULL,
    lap_number INTEGER NOT NULL,
    pitstop_number INTEGER NOT NULL,
    pittime TEXT NOT NULL,
    duration  DECIMAL(6, 3) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    FOREIGN KEY (race_id) REFERENCES races (id),
    FOREIGN KEY (driver_id) REFERENCES drivers (id),
    CONSTRAINT race_driver_pitstop_number UNIQUE (race_id, driver_id, pitstop_number)
);
