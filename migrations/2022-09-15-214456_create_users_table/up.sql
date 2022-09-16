-- Your SQL goes here
CREATE TABLE users (
    id INTEGER PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
)
