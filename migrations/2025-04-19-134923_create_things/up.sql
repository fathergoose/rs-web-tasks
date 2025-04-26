-- Your SQL goes here
CREATE TABLE things (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  details TEXT,
  active BOOLEAN NOT NULL DEFAULT FALSE
)
