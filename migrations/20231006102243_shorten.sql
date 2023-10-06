-- Add migration script here
CREATE TABLE IF NOT EXISTS shorten (
    url VARCHAR(255) NOT NULL,
    shortUrl VARCHAR(255) NOT NULL
);