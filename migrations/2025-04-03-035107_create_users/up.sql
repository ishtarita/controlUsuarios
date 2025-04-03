CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    creationDate DATE DEFAULT now(),
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL
);
