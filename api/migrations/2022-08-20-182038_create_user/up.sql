CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "user" (
	id SERIAL PRIMARY KEY,
    username TEXT NOT NULL,
	password TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP
);
