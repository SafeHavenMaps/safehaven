CREATE TABLE options (
    name VARCHAR(255) NOT NULL,
    value JSONB NOT NULL
);
CREATE UNIQUE INDEX options_name_uindex ON options (name);
