-- Your SQL goes here
CREATE TABLE drivers (
    id TEXT PRIMARY KEY,
    permanent_number INTEGER,
    code TEXT,
    given_name TEXT NOT NULL,
    family_name TEXT NOT NULL,
    date_of_birth DATE NOT NULL,
    nationality TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE constructors (
    id TEXT PRIMARY KEY,
    url TEXT NOT NULL,
    name TEXT NOT NULL,
    nationality TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
