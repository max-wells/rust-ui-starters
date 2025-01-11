-- Add migration script here
CREATE TABLE IF NOT EXISTS todos (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    completed BOOLEAN NOT NULL DEFAULT FALSE
);

INSERT INTO todos (title, description) VALUES
('Learn Rust', 'Complete the Rust book and start a new project'),
('Build a REST API with Rust', 'Use Actix-web and Diesel ORM')