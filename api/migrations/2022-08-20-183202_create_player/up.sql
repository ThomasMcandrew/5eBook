-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "player" (
	id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
	race_id INT NOT NULL,
	alignment_id INT NOT NULL, 
	background_id INT NOT NULL,
	class_id INT NOT NULL,
	user_id INT NOT NULL,
    created_at TIMESTAMP   NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP
);
