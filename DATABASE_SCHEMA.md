# Database Schema Implementation

## Overview
This document verifies that all database schema requirements from task 2 have been implemented with proper domain constraints.

## Migrations Created

### 1. Custom Enums Migration
**File**: `migrations/2024-01-01-000001_create_enums/up.sql`

Creates two custom PostgreSQL enums:
- `brewing_method`: V60, Chemex, FrenchPress, AeroPress, Espresso, Moka, ColdBrew, Other
- `grinder_type`: BurrConical, BurrFlat, Blade, Manual

### 2. Coffee Beans Table Migration  
**File**: `migrations/2024-01-01-000002_create_coffee_beans/up.sql`

Creates `coffee_beans` table with:
- **Inventory tracking**: `remaining_grams` field with CHECK constraint >= 0
- **Required fields**: name, origin, roast_date, purchase_date
- **Optional fields**: variety, processing_method
- **Indexes**: roast_date, origin, remaining_grams (partial index)

### 3. Grinders Table Migration
**File**: `migrations/2024-01-01-000003_create_grinders/up.sql`

Creates `grinders` table with:
- **Setting range validation**: CHECK constraints ensuring min_setting > 0 and max_setting > min_setting
- **Unique constraint**: brand + model combination
- **Grinder type**: Uses custom enum
- **Indexes**: grinder_type, unique brand+model

### 4. Brewing Sessions Table Migration
**File**: `migrations/2024-01-01-000004_create_brewing_sessions/up.sql`

Creates `brewing_sessions` table with comprehensive validation:

#### Domain Constraints Implemented:
1. **Coffee Ratio**: `water_grams / coffee_grams BETWEEN 15.0 AND 17.0`
2. **Rating Increments**: `(rating * 2) = FLOOR(rating * 2)` ensures 0.5 increments
3. **Parameter Ranges**:
   - `coffee_grams BETWEEN 10.0 AND 100.0`
   - `water_grams BETWEEN 150.0 AND 1700.0`
   - `brew_time_seconds BETWEEN 30 AND 480`
   - `water_temp_celsius BETWEEN 60 AND 100`
   - `rating BETWEEN 1.0 AND 10.0`

#### Foreign Key Constraints:
- `bean_id` references `coffee_beans(id)` with RESTRICT on delete
- `grinder_id` references `grinders(id)` with RESTRICT on delete

#### Indexes for Performance:
- Individual indexes on bean_id, grinder_id, created_at, rating, method, grind_setting
- Composite analytics index on (grinder_id, grind_setting, rating)

## Rust Models and Validation

### Schema.rs Generated
**File**: `src/schema.rs`
- Diesel table definitions for all three tables
- Proper joinable relationships
- Custom enum type integration

### Models Implementation
**File**: `src/models/mod.rs`
- `CoffeeBean`, `Grinder`, `BrewingSession` structs with Diesel derives
- `NewCoffeeBean`, `NewGrinder`, `NewBrewingSession` insertable structs
- Custom enum types with diesel-derive-enum integration

### SQL Types
**File**: `src/models/sql_types.rs`
- Custom SQL type definitions for PostgreSQL enums
- Proper Diesel integration

### Validation Layer
**File**: `src/validation.rs`
- All domain validation rules implemented as pure functions
- Comprehensive error types with specific error messages
- Constants for all validation ranges
- Unit tests for all validation rules

## Validation Rules Verified

### Coffee Ratio Validation
```rust
pub fn validate_coffee_ratio(coffee_grams: f32, water_grams: f32) -> Result<()>
```
- Ensures ratio between 15.0 and 17.0 (inclusive)
- Matches database constraint: `water_grams / coffee_grams BETWEEN 15.0 AND 17.0`

### Rating Validation
```rust
pub fn validate_rating(rating: f32) -> Result<()>
```
- Ensures rating between 1.0 and 10.0
- Validates 0.5 increments only (1.0, 1.5, 2.0, ..., 9.5, 10.0)
- Matches database constraint: `(rating * 2) = FLOOR(rating * 2)`

### Parameter Range Validation
All parameter ranges match database CHECK constraints:
- Coffee amount: 10.0-100.0g
- Water amount: 150.0-1700.0ml  
- Brew time: 30-480 seconds
- Water temperature: 60-100°C

### Grind Setting Validation
```rust
pub fn validate_grind_setting(setting: i32, min_setting: i32, max_setting: i32, grinder_name: &str) -> Result<()>
```
- Validates against grinder-specific min/max range
- Provides detailed error messages with grinder name

## Requirements Mapping

### Requirement 1.1 (Record brewing sessions)
✅ `brewing_sessions` table captures all required parameters:
- Coffee bean information (via foreign key)
- Grinder type and grind setting
- Brewing method, water temperature, brew time
- Coffee-to-water ratio
- Tasting notes and rating

### Requirement 1.2 (Parameter validation)
✅ Database-level constraints enforce:
- Valid coffee ratios (15:1 to 17:1)
- Rating increments (0.5 steps)
- Parameter ranges for all brewing variables
- Grind setting validation against grinder ranges

### Requirement 3.1 (Equipment management)
✅ `grinders` table with:
- Setting range validation (min_setting < max_setting)
- Grinder type classification
- Unique brand+model constraint

### Requirement 3.2 (Bean inventory)
✅ `coffee_beans` table with:
- Inventory tracking (remaining_grams with >= 0 constraint)
- Purchase and roast date tracking
- Origin and variety information

## Database Schema Verification

All migrations include proper:
1. **Up migrations**: Create tables with constraints
2. **Down migrations**: Clean rollback procedures  
3. **Indexes**: Performance optimization for common queries
4. **Foreign keys**: Referential integrity
5. **Check constraints**: Domain validation at database level
6. **Enum integration**: Type-safe brewing methods and grinder types

The schema implementation ensures data integrity at the database level while providing a clean Rust API through Diesel ORM integration.