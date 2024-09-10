CREATE TYPE toner_color AS ENUM ('black', 'cyan', 'yellow', 'magenta');

CREATE TABLE toners (
    id UUID PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    color toner_color NOT NULL
);
