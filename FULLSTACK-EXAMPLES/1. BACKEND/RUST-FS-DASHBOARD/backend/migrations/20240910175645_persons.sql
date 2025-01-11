-- Add migration script here
-- Add migration script here
CREATE TABLE IF NOT EXISTS persons (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    level VARCHAR(255) NOT NULL,
    compensation INT NOT NULL,
    joined_date DATE NOT NULL
);

INSERT INTO persons (name, title, level, compensation, joined_date) VALUES 
('Maxime', 'Software Engineer', 'Senior', 80000, '2024-07-11'),
('John', 'Software Engineer', 'Lead', 85000, '2023-01-28'),
('Jane', 'Software Engineer', 'Junior', 60000, '2022-07-16');