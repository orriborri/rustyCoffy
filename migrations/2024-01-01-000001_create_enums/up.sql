-- Create custom enums for brewing methods and grinder types
CREATE TYPE brewing_method AS ENUM (
    'V60',
    'Chemex', 
    'FrenchPress',
    'AeroPress',
    'Espresso',
    'Moka',
    'ColdBrew',
    'Other'
);

CREATE TYPE grinder_type AS ENUM (
    'BurrConical',
    'BurrFlat', 
    'Blade',
    'Manual'
);