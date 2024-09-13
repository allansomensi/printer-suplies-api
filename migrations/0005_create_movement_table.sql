CREATE TABLE movements (
    id UUID PRIMARY KEY,
    printer_id UUID REFERENCES printers(id) ON DELETE SET NULL,
    toner_id UUID REFERENCES toners(id) ON DELETE SET NULL,
    drum_id UUID REFERENCES drums(id) ON DELETE SET NULL,
    quantity INTEGER NOT NULL,
    created_at TIMESTAMPTZ
);