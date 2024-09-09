CREATE TABLE printers (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    model VARCHAR NOT NULL,
    brand UUID NOT NULL,
    toner UUID NOT NULL,
    drum UUID NOT NULL
);