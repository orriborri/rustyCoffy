-- Drop brewing_sessions table and indexes
DROP INDEX IF EXISTS idx_brewing_sessions_analytics;
DROP INDEX IF EXISTS idx_brewing_sessions_grind_setting;
DROP INDEX IF EXISTS idx_brewing_sessions_method;
DROP INDEX IF EXISTS idx_brewing_sessions_rating;
DROP INDEX IF EXISTS idx_brewing_sessions_created_at;
DROP INDEX IF EXISTS idx_brewing_sessions_grinder_id;
DROP INDEX IF EXISTS idx_brewing_sessions_bean_id;
DROP TABLE IF EXISTS brewing_sessions;