-- Schema Verification Script
-- This script can be run against a PostgreSQL database to verify the schema implementation

-- Verify custom enums exist
SELECT typname FROM pg_type WHERE typname IN ('brewing_method', 'grinder_type');

-- Verify tables exist with correct structure
SELECT table_name, column_name, data_type, is_nullable, column_default
FROM information_schema.columns 
WHERE table_name IN ('coffee_beans', 'grinders', 'brewing_sessions')
ORDER BY table_name, ordinal_position;

-- Verify constraints exist
SELECT 
    tc.table_name, 
    tc.constraint_name, 
    tc.constraint_type,
    cc.check_clause
FROM information_schema.table_constraints tc
LEFT JOIN information_schema.check_constraints cc 
    ON tc.constraint_name = cc.constraint_name
WHERE tc.table_name IN ('coffee_beans', 'grinders', 'brewing_sessions')
ORDER BY tc.table_name, tc.constraint_type;

-- Verify foreign key relationships
SELECT 
    kcu.table_name,
    kcu.column_name,
    ccu.table_name AS foreign_table_name,
    ccu.column_name AS foreign_column_name
FROM information_schema.key_column_usage kcu
JOIN information_schema.constraint_column_usage ccu
    ON kcu.constraint_name = ccu.constraint_name
WHERE kcu.table_name IN ('brewing_sessions');

-- Verify indexes exist
SELECT 
    schemaname,
    tablename,
    indexname,
    indexdef
FROM pg_indexes 
WHERE tablename IN ('coffee_beans', 'grinders', 'brewing_sessions')
ORDER BY tablename, indexname;