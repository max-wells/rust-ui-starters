-- Add migration script here
-- Add migration script here
CREATE TABLE IF NOT EXISTS xxxs (
    id SERIAL PRIMARY KEY,
    field_one VARCHAR(255) NOT NULL,
    field_two VARCHAR(255) NOT NULL
);

INSERT INTO xxxs (field_one, field_two) VALUES ('My field 1 - A', 'My field 2 - B');
INSERT INTO xxxs (field_one, field_two) VALUES ('My field 1 - C', 'My field 2 - D');
INSERT INTO xxxs (field_one, field_two) VALUES ('My field 1 - E', 'My field 2 - F');
