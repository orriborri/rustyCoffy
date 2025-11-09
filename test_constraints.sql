-- Test Data Script to Verify Database Constraints
-- Run this after applying migrations to test constraint validation

-- Test 1: Insert valid coffee bean
INSERT INTO coffee_beans (name, origin, roast_date, purchase_date, remaining_grams, variety)
VALUES ('Ethiopian Yirgacheffe', 'Ethiopia', '2024-01-15', '2024-01-20', 250.0, 'Heirloom');

-- Test 2: Insert valid grinder
INSERT INTO grinders (brand, model, grinder_type, min_setting, max_setting)
VALUES ('Baratza', 'Encore', 'BurrConical', 1, 40);

-- Test 3: Insert valid brewing session
INSERT INTO brewing_sessions (
    bean_id, grinder_id, grind_setting, brewing_method,
    coffee_grams, water_grams, rating, tasting_notes
) VALUES (
    1, 1, 15, 'V60',
    20.0, 320.0, 8.5, 'Bright and fruity with floral notes'
);

-- Test 4: Try to violate coffee ratio constraint (should fail)
-- This should fail with coffee ratio constraint violation
-- INSERT INTO brewing_sessions (
--     bean_id, grinder_id, grind_setting, brewing_method,
--     coffee_grams, water_grams, rating
-- ) VALUES (
--     1, 1, 15, 'V60',
--     20.0, 200.0, 7.0  -- 10:1 ratio, violates 15:1-17:1 constraint
-- );

-- Test 5: Try to violate rating increment constraint (should fail)
-- This should fail with rating increment constraint violation
-- INSERT INTO brewing_sessions (
--     bean_id, grinder_id, grind_setting, brewing_method,
--     coffee_grams, water_grams, rating
-- ) VALUES (
--     1, 1, 15, 'V60',
--     20.0, 320.0, 7.3  -- Not in 0.5 increments
-- );

-- Test 6: Try to violate grinder setting range (should fail)
-- This should fail with grind setting out of range
-- INSERT INTO brewing_sessions (
--     bean_id, grinder_id, grind_setting, brewing_method,
--     coffee_grams, water_grams, rating
-- ) VALUES (
--     1, 1, 50, 'V60',  -- 50 > max_setting (40)
--     20.0, 320.0, 8.0
-- );

-- Test 7: Try negative remaining grams (should fail)
-- This should fail with remaining_grams constraint violation
-- INSERT INTO coffee_beans (name, origin, roast_date, purchase_date, remaining_grams)
-- VALUES ('Test Bean', 'Test Origin', '2024-01-01', '2024-01-01', -10.0);

-- Test 8: Try invalid grinder settings (should fail)
-- This should fail with min_setting > max_setting constraint violation
-- INSERT INTO grinders (brand, model, grinder_type, min_setting, max_setting)
-- VALUES ('Test', 'Grinder', 'Blade', 20, 10);  -- min > max

-- Verify successful inserts
SELECT 'Coffee Beans' as table_name, COUNT(*) as record_count FROM coffee_beans
UNION ALL
SELECT 'Grinders', COUNT(*) FROM grinders  
UNION ALL
SELECT 'Brewing Sessions', COUNT(*) FROM brewing_sessions;