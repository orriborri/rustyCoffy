# Brewing Service Documentation

## Overview

The `BrewingService` is the business logic layer that wraps database operations with comprehensive validation and provides a high-level API for the coffee brewing tracker application.

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    BrewingService                        │
│  (Business Logic Layer with Validation)                 │
├─────────────────────────────────────────────────────────┤
│  - Session Management                                    │
│  - Bean Inventory Management                            │
│  - Grinder Equipment Management                         │
│  - Validation & Error Handling                          │
└─────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────┐
│                    Database Layer                        │
│  (CRUD Operations with Connection Pooling)              │
├─────────────────────────────────────────────────────────┤
│  - Diesel ORM                                           │
│  - r2d2 Connection Pool                                 │
│  - PostgreSQL Database                                  │
└─────────────────────────────────────────────────────────┘
```

## Key Features

### 1. Session Management

The service provides comprehensive brewing session management with automatic validation:

```rust
// Create a new brewing session
let session_id = service.create_session(NewBrewingSession {
    bean_id: 1,
    grinder_id: 1,
    grind_setting: 20,
    brewing_method: BrewingMethod::V60.to_string(),
    water_temp_celsius: Some(95),
    brew_time_seconds: Some(240),
    coffee_grams: 20.0,
    water_grams: 320.0,
    tasting_notes: Some("Bright and fruity".to_string()),
    rating: Some(8.5),
})?;
```

**Validation performed:**
- Coffee-to-water ratio must be between 15:1 and 17:1
- Rating must be in 0.5 increments from 1.0 to 10.0
- Grind setting must be within grinder's min/max range
- Bean must exist and have sufficient quantity
- Grinder must exist
- All parameter ranges are validated

**Automatic operations:**
- Bean quantity is automatically decremented
- Foreign key relationships are validated
- Timestamps are automatically set

### 2. Session Filtering and Search

Filter sessions by multiple criteria:

```rust
// Filter by rating and brewing method
let filter = SessionFilter {
    rating_min: Some(8.0),
    brewing_method: Some(BrewingMethod::V60),
    limit: Some(50),
    ..Default::default()
};

let sessions = service.get_sessions(filter)?;
```

**Available filters:**
- `bean_origin`: Filter by coffee bean origin
- `roast_date_from` / `roast_date_to`: Filter by roast date range
- `grinder_type`: Filter by grinder type
- `brewing_method`: Filter by brewing method
- `rating_min`: Filter by minimum rating
- `limit`: Limit number of results

Search sessions by text:

```rust
// Search across bean names, origins, grinder brands, and tasting notes
let results = service.search_sessions("Ethiopian")?;
```

### 3. Session Duplication

Reproduce successful brews by duplicating sessions:

```rust
// Duplicate a session (copies parameters but not rating/notes)
let new_session_id = service.duplicate_session(original_session_id)?;
```

**Duplication behavior:**
- All brewing parameters are copied
- Rating and tasting notes are NOT copied (set to None)
- Bean quantity is automatically decremented
- New timestamp is set

### 4. Bean Inventory Management

Track coffee beans with automatic quantity management:

```rust
// Add a new bean
let bean_id = service.add_bean(NewCoffeeBean {
    name: "Ethiopian Yirgacheffe".to_string(),
    origin: "Ethiopia".to_string(),
    roast_date: NaiveDate::from_ymd(2024, 1, 15),
    purchase_date: NaiveDate::from_ymd(2024, 1, 20),
    remaining_grams: Some(500.0),
    variety: Some("Heirloom".to_string()),
    processing_method: Some("Washed".to_string()),
})?;

// Get only active beans (with remaining quantity > 0)
let active_beans = service.get_active_beans()?;

// Get usage history for a bean
let history = service.get_bean_usage_history(bean_id)?;
```

**Features:**
- Automatic quantity tracking when creating sessions
- Validation of sufficient quantity before session creation
- Usage history tracking
- Active bean filtering

### 5. Grinder Equipment Management

Manage grinders with usage statistics:

```rust
// Add a new grinder
let grinder_id = service.add_grinder(NewGrinder {
    brand: "Baratza".to_string(),
    model: "Encore".to_string(),
    grinder_type: GrinderType::BurrConical.to_string(),
    min_setting: 10,
    max_setting: 40,
})?;

// Get usage statistics
let stats = service.get_grinder_usage_stats(grinder_id)?;
println!("Total sessions: {}", stats.total_sessions);
println!("Average rating: {:?}", stats.average_rating);
println!("Most used setting: {:?}", stats.most_used_setting);

// Validate a grind setting
service.validate_grind_setting(grinder_id, 25)?;
```

**Statistics provided:**
- Total number of sessions
- Average rating across all sessions
- Most frequently used grind setting

## Validation Rules

### Coffee Ratio Validation

```rust
// Valid: 15:1 to 17:1 ratio
coffee_grams: 20.0, water_grams: 320.0  // 16:1 ✓
coffee_grams: 20.0, water_grams: 300.0  // 15:1 ✓
coffee_grams: 20.0, water_grams: 340.0  // 17:1 ✓

// Invalid: Outside range
coffee_grams: 20.0, water_grams: 280.0  // 14:1 ✗
coffee_grams: 20.0, water_grams: 360.0  // 18:1 ✗
```

### Rating Validation

```rust
// Valid: 0.5 increments from 1.0 to 10.0
rating: Some(1.0)   ✓
rating: Some(5.5)   ✓
rating: Some(10.0)  ✓

// Invalid: Not in 0.5 increments or out of range
rating: Some(0.5)   ✗  // Too low
rating: Some(5.3)   ✗  // Not 0.5 increment
rating: Some(10.5)  ✗  // Too high
```

### Grind Setting Validation

```rust
// Grinder with range 10-40
grind_setting: 15   ✓
grind_setting: 40   ✓

grind_setting: 5    ✗  // Below minimum
grind_setting: 50   ✗  // Above maximum
```

### Parameter Ranges

- **Coffee Amount**: 10.0 - 100.0 grams
- **Water Amount**: 150.0 - 1700.0 ml
- **Brew Time**: 30 - 480 seconds
- **Water Temperature**: 60 - 100°C
- **Coffee Ratio**: 15.0 - 17.0 (water/coffee)
- **Rating**: 1.0 - 10.0 in 0.5 increments

## Error Handling

The service uses the `BrewingError` enum for comprehensive error handling:

```rust
pub enum BrewingError {
    BeanNotFound(i32),
    GrinderNotFound(i32),
    GrindSettingOutOfRange { setting: i32, min: i32, max: i32, grinder: String },
    CoffeeRatioInvalid { ratio: f32, min: f32, max: f32 },
    InvalidQualityRating { rating: f32 },
    InvalidCoffeeAmount { amount: f32, min: f32, max: f32 },
    InvalidWaterAmount { amount: f32, min: f32, max: f32 },
    InvalidBrewTime { time: i32, min: i32, max: i32 },
    InvalidWaterTemperature { temp: i32, min: i32, max: i32 },
    InsufficientBeanQuantity { available: f32, required: f32 },
    InvalidGrinderSettings { message: String },
    InvalidBeanData { message: String },
    Database(diesel::result::Error),
    Pool(r2d2::Error),
}
```

**Error handling example:**

```rust
match service.create_session(session) {
    Ok(session_id) => println!("Session created: {}", session_id),
    Err(BrewingError::InsufficientBeanQuantity { available, required }) => {
        println!("Not enough beans! Need {}g but only {}g available", required, available);
    }
    Err(BrewingError::GrindSettingOutOfRange { setting, min, max, grinder }) => {
        println!("Grind setting {} is outside range {}-{} for {}", setting, min, max, grinder);
    }
    Err(e) => println!("Error: {}", e),
}
```

## Testing Strategy

### Unit Tests

Located in `src/services/brewing_service_tests.rs`:

- Property-based tests using `proptest` for validation logic
- Edge case testing for all validation rules
- Model method testing (coffee_ratio, is_high_quality, etc.)

### Integration Tests

Located in `src/services/brewing_service_integration_tests.rs`:

- Complete workflow tests (add equipment → create session → verify results)
- Session filtering and search functionality
- Session duplication
- Grinder usage statistics
- Bean usage history
- Error condition testing

### Mutation Testing

Run mutation tests to verify test quality:

```bash
./run_mutation_tests.sh
```

This uses `cargo-mutants` to introduce mutations in the code and verify that tests catch them.

## Usage Examples

### Complete Workflow Example

```rust
use coffee_brewing_tracker::services::BrewingService;
use coffee_brewing_tracker::models::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize service
    let service = BrewingService::new("postgresql://localhost/coffee_tracker")?;
    
    // Add equipment
    let bean_id = service.add_bean(NewCoffeeBean {
        name: "Ethiopian Yirgacheffe".to_string(),
        origin: "Ethiopia".to_string(),
        roast_date: chrono::Utc::now().naive_utc().date(),
        purchase_date: chrono::Utc::now().naive_utc().date(),
        remaining_grams: Some(500.0),
        variety: Some("Heirloom".to_string()),
        processing_method: Some("Washed".to_string()),
    })?;
    
    let grinder_id = service.add_grinder(NewGrinder {
        brand: "Baratza".to_string(),
        model: "Encore".to_string(),
        grinder_type: GrinderType::BurrConical.to_string(),
        min_setting: 10,
        max_setting: 40,
    })?;
    
    // Create brewing session
    let session_id = service.create_session(NewBrewingSession {
        bean_id,
        grinder_id,
        grind_setting: 20,
        brewing_method: BrewingMethod::V60.to_string(),
        water_temp_celsius: Some(95),
        brew_time_seconds: Some(240),
        coffee_grams: 20.0,
        water_grams: 320.0,
        tasting_notes: Some("Bright and fruity with floral notes".to_string()),
        rating: Some(8.5),
    })?;
    
    println!("Created session: {}", session_id);
    
    // Get high-rated sessions
    let filter = SessionFilter {
        rating_min: Some(8.0),
        ..Default::default()
    };
    
    let high_rated = service.get_sessions(filter)?;
    println!("Found {} high-rated sessions", high_rated.len());
    
    // Duplicate a successful brew
    let duplicated_id = service.duplicate_session(session_id)?;
    println!("Duplicated session: {}", duplicated_id);
    
    Ok(())
}
```

## Performance Considerations

- **Connection Pooling**: Uses r2d2 with a pool size of 10 connections
- **Query Optimization**: Indexes on frequently queried columns (bean_id, grinder_id, rating, created_at)
- **Default Limits**: Session queries default to 100 results to prevent large result sets
- **Efficient Filtering**: Database-level filtering using Diesel's query builder

## Future Enhancements

Potential improvements for the service layer:

1. **Caching**: Add caching for frequently accessed data (beans, grinders)
2. **Batch Operations**: Support bulk session creation
3. **Async Support**: Convert to async/await for better concurrency
4. **Analytics**: Add more sophisticated statistical analysis
5. **Export/Import**: Add data export and import functionality
6. **Notifications**: Add event notifications for low bean quantities
7. **Recommendations**: ML-based brewing recommendations

## API Reference

### Session Management

- `create_session(session: NewBrewingSession) -> Result<i32>`
- `get_sessions(filter: SessionFilter) -> Result<Vec<BrewingSession>>`
- `get_all_sessions() -> Result<Vec<BrewingSession>>`
- `get_session_by_id(id: i32) -> Result<Option<BrewingSession>>`
- `duplicate_session(id: i32) -> Result<i32>`
- `search_sessions(query: &str) -> Result<Vec<BrewingSession>>`

### Bean Management

- `add_bean(bean: NewCoffeeBean) -> Result<i32>`
- `get_beans() -> Result<Vec<CoffeeBean>>`
- `get_active_beans() -> Result<Vec<CoffeeBean>>`
- `get_bean_by_id(id: i32) -> Result<Option<CoffeeBean>>`
- `update_bean_quantity(id: i32, grams_used: f32) -> Result<()>`
- `get_bean_usage_history(bean_id: i32) -> Result<Vec<BrewingSession>>`

### Grinder Management

- `add_grinder(grinder: NewGrinder) -> Result<i32>`
- `get_grinders() -> Result<Vec<Grinder>>`
- `get_grinder_by_id(id: i32) -> Result<Option<Grinder>>`
- `get_grinder_usage_stats(grinder_id: i32) -> Result<GrinderStats>`
- `validate_grind_setting(grinder_id: i32, setting: i32) -> Result<()>`

## Conclusion

The `BrewingService` provides a robust, well-tested business logic layer with comprehensive validation, automatic quantity tracking, and rich querying capabilities. It serves as the primary interface for all brewing-related operations in the application.
