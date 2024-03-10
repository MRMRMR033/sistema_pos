-- Your SQL goes here
CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    lastname VARCHAR NOT NULL,
    email VARCHAR NOT NULL UNIQUE,
    cel VARCHAR NOT NULL UNIQUE,
    house_cel VARCHAR NOT NULL,
    active BOOLEAN NOT NULL DEFAULT FALSE,
    admin BOOLEAN NOT NULL DEFAULT FALSE
);