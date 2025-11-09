-- Create coffee_beans table with inventory tracking constraints
CREATE TABLE coffee_beans (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    origin VARCHAR NOT NULL,
    roast_date DATE NOT NULL,
    purchase_date DATE NOT NULL,
    remaining_grams REAL CHECK (remaining_grams >= 0),
    variety VARCHAR,
    processing_method VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for efficient querying
CREATE INDEX idx_coffee_beans_roast_date ON coffee_beans(roast_date);
CREATE INDEX idx_coffee_beans_origin ON coffee_beans(origin);
CREATE INDEX idx_coffee_beans_remaining_grams ON coffee_beans(remaining_grams) WHERE remaining_grams IS NOT NULL;