-- Drop coffee_beans table and indexes
DROP INDEX IF EXISTS idx_coffee_beans_remaining_grams;
DROP INDEX IF EXISTS idx_coffee_beans_origin;
DROP INDEX IF EXISTS idx_coffee_beans_roast_date;
DROP TABLE IF EXISTS coffee_beans;