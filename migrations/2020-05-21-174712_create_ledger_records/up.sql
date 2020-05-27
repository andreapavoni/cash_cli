-- Your SQL goes here
CREATE TABLE records (
    id INTEGER NOT NULL PRIMARY KEY,
    amount INTEGER NOT NULL,
    category VARCHAR NOT NULL,
    label VARCHAR NOT NULL,
    date DATE NOT NULL,
    operation VARCHAR NOT NULL
)
