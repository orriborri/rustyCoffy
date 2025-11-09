-- Create grinders table with setting range validation
CREATE TABLE grinders (
    id SERIAL PRIMARY KEY,
    brand VARCHAR NOT NULL,
    model VARCHAR NOT NULL,
    grinder_type grinder_type NOT NULL,
    min_setting INTEGER NOT NULL CHECK (min_setting > 0),
    max_setting INTEGER NOT NULL CHECK (max_setting > min_setting),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create unique index to prevent duplicate grinder models
CREATE UNIQUE INDEX idx_grinders_brand_model ON grinders(brand, model);

-- Create index for grinder type queries
CREATE INDEX idx_grinders_type ON grinders(grinder_type);