CREATE TABLE IF NOT EXISTS conversions
(
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    formula VARCHAR(255) NOT NULL
);