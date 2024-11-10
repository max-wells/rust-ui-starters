-- Add migration script here
-- Add migration script here
CREATE TABLE IF NOT EXISTS books (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL
);

INSERT INTO books (title, author) VALUES ('The Great Gatsby', 'F. Scott Fitzgerald');
INSERT INTO books (title, author) VALUES ('The Little Prince', 'Antoine de Saint-Exupéry');
INSERT INTO books (title, author) VALUES ('1984', 'George Orwell');
