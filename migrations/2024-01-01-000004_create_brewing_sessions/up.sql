-- Create brewing_sessions table with comprehensive validation constraints
CREATE TABLE brewing_sessions (
    id SERIAL PRIMARY KEY,
    bean_id INTEGER NOT NULL,
    grinder_id INTEGER NOT NULL,
    grind_setting INTEGER NOT NULL,
    brewing_method brewing_method NOT NULL,
    water_temp_celsius INTEGER CHECK (water_temp_celsius BETWEEN 60 AND 100),
    brew_time_seconds INTEGER CHECK (brew_time_seconds BETWEEN 30 AND 480),
    coffee_grams REAL NOT NULL CHECK (coffee_grams BETWEEN 10.0 AND 100.0),
    water_grams REAL NOT NULL CHECK (water_grams BETWEEN 150.0 AND 1700.0),
    tasting_notes TEXT,
    rating REAL CHECK (
        rating >= 1.0 AND 
        rating <= 10.0 AND 
        (rating * 2) = FLOOR(rating * 2)  -- Ensures 0.5 increments
    ),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Foreign key constraints
    FOREIGN KEY (bean_id) REFERENCES coffee_beans (id) ON DELETE RESTRICT,
    FOREIGN KEY (grinder_id) REFERENCES grinders (id) ON DELETE RESTRICT,
    
    -- Coffee ratio constraint (15.0 to 17.0 ratio)
    CONSTRAINT valid_coffee_ratio CHECK (water_grams / coffee_grams BETWEEN 15.0 AND 17.0)
);

-- Create indexes for efficient querying
CREATE INDEX idx_brewing_sessions_bean_id ON brewing_sessions(bean_id);
CREATE INDEX idx_brewing_sessions_grinder_id ON brewing_sessions(grinder_id);
CREATE INDEX idx_brewing_sessions_created_at ON brewing_sessions(created_at);
CREATE INDEX idx_brewing_sessions_rating ON brewing_sessions(rating) WHERE rating IS NOT NULL;
CREATE INDEX idx_brewing_sessions_method ON brewing_sessions(brewing_method);
CREATE INDEX idx_brewing_sessions_grind_setting ON brewing_sessions(grind_setting);

-- Composite index for analytics queries
CREATE INDEX idx_brewing_sessions_analytics ON brewing_sessions(grinder_id, grind_setting, rating) WHERE rating IS NOT NULL;