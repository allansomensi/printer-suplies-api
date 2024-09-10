CREATE TABLE printers (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    model VARCHAR(50) NOT NULL,
    brand UUID NOT NULL,
    toner UUID NOT NULL,
    drum UUID NOT NULL,
    FOREIGN KEY (brand) REFERENCES brands(id),
    FOREIGN KEY (toner) REFERENCES toners(id),
    FOREIGN KEY (drum) REFERENCES drums(id)
);
