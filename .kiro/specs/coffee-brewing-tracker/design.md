# Design Document

## Overview

The coffee brewing tracker will be built as a Rust application using Dioxus for the frontend interface with local data persistence. The application will use a modular architecture with clear separation between data models, business logic, and UI components. Data will be stored in a local Postgres database for simplicity and portability. Dioxus will allow us to target both web and desktop platforms from the same codebase.

The core focus is on tracking grind size optimization, as the optimal grind setting varies based on brewing method and coffee bean characteristics. This relationship between grinder settings, brewing equipment, and bean properties is central to achieving consistent, high-quality brews and forms the foundation of the application's analytics capabilities.

## Architecture

### High-Level Architecture
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Dioxus Frontend│────│  Business Logic │────│   Data Layer    │
│  (Components)   │    │   (Services)    │    │   (Postgres)      │
└─────────────────┘    └─────────────────┘    └──────────────f───┘
```

### Technology Stack
- **Frontend Framework**: `dioxus` for reactive UI components
- **Web Target**: `dioxus-web` for browser deployment
- **Desktop Target**: `dioxus-desktop` for native app (future enhancement)
- **ORM**: `diesel` for type-safe database operations and migrations
- **Database**: Postgres for local storage with full ACID compliance
- **Connection Pooling**: `r2d2` with `diesel` for connection management
- **Serialization**: `serde` with `serde_json` for data handling
- **Date/Time**: `chrono` for timestamp handling
- **Error Handling**: `anyhow` for application errors, `thiserror` for domain errors
- **Testing**: `proptest` for property-based testing, `mockall` for mocking, `cargo-mutants` for mutation testing
- **Styling**: `dioxus-css` or inline styles with Tailwind-like utilities

## Components and Interfaces

### Core Models (Diesel)
```rust
use diesel::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Serialize, Deserialize};

// Coffee bean information with inventory tracking
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = coffee_beans)]
pub struct CoffeeBean {
    pub id: i32,
    pub name: String,
    pub origin: String,
    pub roast_date: NaiveDate,
    pub purchase_date: NaiveDate,
    pub remaining_grams: Option<f32>,
    pub variety: Option<String>,
    pub processing_method: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = coffee_beans)]
pub struct NewCoffeeBean {
    pub name: String,
    pub origin: String,
    pub roast_date: NaiveDate,
    pub purchase_date: NaiveDate,
    pub remaining_grams: Option<f32>,
    pub variety: Option<String>,
    pub processing_method: Option<String>,
}

// Grinder equipment with setting ranges for validation
#[derive(Queryable, Identifiable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = grinders)]
pub struct Grinder {
    pub id: i32,
    pub brand: String,
    pub model: String,
    pub grinder_type: GrinderType,
    pub min_setting: i32,
    pub max_setting: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = grinders)]
pub struct NewGrinder {
    pub brand: String,
    pub model: String,
    pub grinder_type: GrinderType,
    pub min_setting: i32,
    pub max_setting: i32,
}

// Individual brewing session with comprehensive tracking
#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize, Debug, Clone)]
#[diesel(belongs_to(CoffeeBean, foreign_key = bean_id))]
#[diesel(belongs_to(Grinder, foreign_key = grinder_id))]
#[diesel(table_name = brewing_sessions)]
pub struct BrewingSession {
    pub id: i32,
    pub bean_id: i32,
    pub grinder_id: i32,
    pub grind_setting: i32,
    pub brewing_method: BrewingMethod,
    pub water_temp_celsius: Option<i32>,
    pub brew_time_seconds: Option<i32>,
    pub coffee_grams: f32,
    pub water_grams: f32,
    pub tasting_notes: Option<String>,
    pub rating: Option<f32>, // Changed to f32 for 0.5 increments
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = brewing_sessions)]
pub struct NewBrewingSession {
    pub bean_id: i32,
    pub grinder_id: i32,
    pub grind_setting: i32,
    pub brewing_method: BrewingMethod,
    pub water_temp_celsius: Option<i32>,
    pub brew_time_seconds: Option<i32>,
    pub coffee_grams: f32,
    pub water_grams: f32,
    pub tasting_notes: Option<String>,
    pub rating: Option<f32>,
}
```

### Service Layer
```rust
use anyhow::Result;

// Main application service with comprehensive brewing management
pub struct BrewingService {
    db: Database,
}

impl BrewingService {
    // Session management with validation and filtering
    pub fn create_session(&mut self, session: NewBrewingSession) -> Result<i32>;
    pub fn get_sessions(&self, filter: SessionFilter) -> Result<Vec<BrewingSession>>;
    pub fn get_session_by_id(&self, id: i32) -> Result<Option<BrewingSession>>;
    pub fn duplicate_session(&mut self, id: i32) -> Result<i32>;
    pub fn search_sessions(&self, query: &str) -> Result<Vec<BrewingSession>>;
    
    // Bean inventory management with automatic quantity tracking
    pub fn add_bean(&mut self, bean: NewCoffeeBean) -> Result<i32>;
    pub fn get_beans(&self) -> Result<Vec<CoffeeBean>>;
    pub fn get_active_beans(&self) -> Result<Vec<CoffeeBean>>; // Beans with remaining quantity
    pub fn update_bean_quantity(&mut self, id: i32, grams_used: f32) -> Result<()>;
    pub fn get_bean_usage_history(&self, bean_id: i32) -> Result<Vec<BrewingSession>>;
    
    // Grinder equipment management with usage statistics
    pub fn add_grinder(&mut self, grinder: NewGrinder) -> Result<i32>;
    pub fn get_grinders(&self) -> Result<Vec<Grinder>>;
    pub fn get_grinder_usage_stats(&self, grinder_id: i32) -> Result<GrinderStats>;
    pub fn validate_grind_setting(&self, grinder_id: i32, setting: i32) -> Result<()>;
    
    // Analytics and statistics for optimization insights
    pub fn get_statistics(&self) -> Result<BrewingStatistics>;
    pub fn get_rating_trends(&self) -> Result<Vec<RatingTrend>>;
    pub fn get_optimal_grind_suggestions(&self, bean_id: i32, method: BrewingMethod) -> Result<Vec<GrindSuggestion>>;
    pub fn get_correlation_analysis(&self) -> Result<CorrelationData>;
}

// Filter struct for session queries
#[derive(Debug, Clone)]
pub struct SessionFilter {
    pub bean_origin: Option<String>,
    pub roast_date_from: Option<NaiveDate>,
    pub roast_date_to: Option<NaiveDate>,
    pub grinder_type: Option<GrinderType>,
    pub brewing_method: Option<BrewingMethod>,
    pub rating_min: Option<f32>,
    pub limit: Option<i64>,
}

// Statistics and analytics structures
#[derive(Debug, Serialize, Deserialize)]
pub struct BrewingStatistics {
    pub total_sessions: i64,
    pub average_rating: f32,
    pub favorite_beans: Vec<BeanStats>,
    pub favorite_methods: Vec<MethodStats>,
    pub grinder_performance: Vec<GrinderStats>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrindSuggestion {
    pub grind_setting: i32,
    pub average_rating: f32,
    pub session_count: i64,
    pub confidence_score: f32,
}
```

### Dioxus Components
```rust
// Main application component with routing and state management
#[component]
fn App() -> Element {
    // Global state management and routing setup
}

// Page components following clean architecture
#[component]
fn Dashboard() -> Element;

#[component] 
fn NewBrewSession() -> Element;

#[component]
fn BrowseSessions() -> Element;

#[component]
fn EquipmentManager() -> Element;

#[component]
fn Statistics() -> Element;

// Reusable UI components with proper props
#[derive(Props, PartialEq)]
struct BrewingSessionCardProps {
    session: BrewingSession,
    on_duplicate: EventHandler<i32>,
    on_view_details: EventHandler<i32>,
}

#[component]
fn BrewingSessionCard(props: BrewingSessionCardProps) -> Element;

#[derive(Props, PartialEq)]
struct BeanSelectorProps {
    beans: Vec<CoffeeBean>,
    selected_bean_id: Option<i32>,
    on_select: EventHandler<i32>,
    show_inventory: bool,
}

#[component]
fn BeanSelector(props: BeanSelectorProps) -> Element;

#[derive(Props, PartialEq)]
struct GrinderSelectorProps {
    grinders: Vec<Grinder>,
    selected_grinder_id: Option<i32>,
    on_select: EventHandler<i32>,
    on_grind_setting_change: EventHandler<i32>,
}

#[component]
fn GrinderSelector(props: GrinderSelectorProps) -> Element;

#[derive(Props, PartialEq)]
struct RatingInputProps {
    rating: Option<f32>,
    on_change: EventHandler<f32>,
    show_visual_feedback: bool, // Green for ratings >= 7.0
}

#[component]
fn RatingInput(props: RatingInputProps) -> Element;

#[derive(Props, PartialEq)]
struct SessionFilterProps {
    filter: SessionFilter,
    on_filter_change: EventHandler<SessionFilter>,
    available_origins: Vec<String>,
    available_methods: Vec<BrewingMethod>,
}

#[component]
fn SessionFilter(props: SessionFilterProps) -> Element;
```

### State Management
```rust
// Global application state with comprehensive data management
#[derive(Clone)]
pub struct AppState {
    pub brewing_service: Rc<RefCell<BrewingService>>,
    pub current_route: Route,
    pub sessions: Vec<BrewingSession>,
    pub beans: Vec<CoffeeBean>,
    pub grinders: Vec<Grinder>,
    pub statistics: Option<BrewingStatistics>,
    pub loading: bool,
    pub error: Option<String>,
}

// Default brewing parameters for consistent UX
#[derive(Clone)]
pub struct BrewingDefaults {
    pub coffee_amount: f32,    // 20.0 grams
    pub water_amount: f32,     // 320.0 ml (maintains 16:1 ratio)
    pub brew_time: i32,        // 240 seconds (4 minutes)
    pub method: BrewingMethod, // PourOver
}

// Use Dioxus hooks for reactive state management
use_shared_state::<AppState>();
use_signal(|| default_value); // For local component state
use_future(|| async { /* load initial data */ }); // For async operations

// Form state management for multi-step brewing session creation
#[derive(Clone, Default)]
pub struct NewSessionState {
    pub selected_bean_id: Option<i32>,
    pub selected_grinder_id: Option<i32>,
    pub grind_setting: Option<i32>,
    pub brewing_method: BrewingMethod,
    pub coffee_amount: f32,
    pub water_amount: f32,
    pub brew_time: Option<i32>,
    pub water_temp: Option<i32>,
    pub tasting_notes: String,
    pub rating: Option<f32>,
    pub validation_errors: Vec<String>,
}
```

### Frontend Interface
The Dioxus interface will consist of several components designed for optimal brewing workflow:
- **Dashboard**: Overview of recent brews, quick stats, and brewing suggestions
- **New Brew**: Step-by-step form following the sequence: Bean Selection → Grinder Selection → Grind Setting → Brew Parameters → Quality Rating
- **Browse Sessions**: Filterable and searchable list of previous brewing sessions with duplication capability
- **Equipment**: Comprehensive management of coffee beans (with inventory tracking) and grinders (with usage statistics)
- **Statistics**: Interactive charts showing grind size optimization, rating trends, and correlation analysis between parameters and quality

## Data Models

### Database Schema (Diesel Migrations)
Diesel will manage the schema through migrations. Initial migration files:

**migrations/2024-01-01-000001_create_enums/up.sql:**
```sql
CREATE TYPE brewing_method AS ENUM ('V60', 'Chemex', 'FrenchPress', 'AeroPress', 'Espresso', 'Moka', 'ColdBrew', 'Other');
CREATE TYPE grinder_type AS ENUM ('BurrConical', 'BurrFlat', 'Blade', 'Manual');
```

**migrations/2024-01-01-000002_create_coffee_beans/up.sql:**
```sql
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

CREATE INDEX idx_coffee_beans_roast_date ON coffee_beans(roast_date);
CREATE INDEX idx_coffee_beans_origin ON coffee_beans(origin);
```

**migrations/2024-01-01-000003_create_grinders/up.sql:**
```sql
CREATE TABLE grinders (
    id SERIAL PRIMARY KEY,
    brand VARCHAR NOT NULL,
    model VARCHAR NOT NULL,
    grinder_type grinder_type NOT NULL,
    min_setting INTEGER NOT NULL CHECK (min_setting > 0),
    max_setting INTEGER NOT NULL CHECK (max_setting > min_setting),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX idx_grinders_brand_model ON grinders(brand, model);
```

**migrations/2024-01-01-000004_create_brewing_sessions/up.sql:**
```sql
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
    rating REAL CHECK (rating >= 1.0 AND rating <= 10.0 AND (rating * 2) = FLOOR(rating * 2)),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (bean_id) REFERENCES coffee_beans (id) ON DELETE RESTRICT,
    FOREIGN KEY (grinder_id) REFERENCES grinders (id) ON DELETE RESTRICT,
    CONSTRAINT valid_coffee_ratio CHECK (water_grams / coffee_grams BETWEEN 15.0 AND 17.0)
);

CREATE INDEX idx_brewing_sessions_bean_id ON brewing_sessions(bean_id);
CREATE INDEX idx_brewing_sessions_grinder_id ON brewing_sessions(grinder_id);
CREATE INDEX idx_brewing_sessions_created_at ON brewing_sessions(created_at);
CREATE INDEX idx_brewing_sessions_rating ON brewing_sessions(rating);
CREATE INDEX idx_brewing_sessions_method ON brewing_sessions(brewing_method);
CREATE INDEX idx_brewing_sessions_grind_setting ON brewing_sessions(grind_setting);
```

### Enums and Types
```rust
use diesel_derive_enum::DbEnum;

#[derive(Debug, Clone, Serialize, Deserialize, DbEnum, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::BrewingMethod"]
pub enum BrewingMethod {
    V60,
    Chemex,
    FrenchPress,
    AeroPress,
    Espresso,
    Moka,
    ColdBrew,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, DbEnum, PartialEq)]
#[ExistingTypePath = "crate::schema::sql_types::GrinderType"]
pub enum GrinderType {
    BurrConical,
    BurrFlat,
    Blade,
    Manual,
}

// Domain validation constraints
pub const COFFEE_RATIO_MIN: f32 = 15.0;
pub const COFFEE_RATIO_MAX: f32 = 17.0;
pub const RATING_MIN: f32 = 1.0;
pub const RATING_MAX: f32 = 10.0;
pub const RATING_INCREMENT: f32 = 0.5;
pub const COFFEE_AMOUNT_MIN: f32 = 10.0;
pub const COFFEE_AMOUNT_MAX: f32 = 100.0;
pub const WATER_AMOUNT_MIN: f32 = 150.0;
pub const WATER_AMOUNT_MAX: f32 = 1700.0;
pub const BREW_TIME_MIN: i32 = 30;
pub const BREW_TIME_MAX: i32 = 480;
```

## Error Handling

### Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum BrewingError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
    
    #[error("Connection pool error: {0}")]
    Pool(#[from] r2d2::Error),
    
    #[error("Bean not found with id: {0}")]
    BeanNotFound(i32),
    
    #[error("Grinder not found with id: {0}")]
    GrinderNotFound(i32),
    
    #[error("Grind setting {setting} outside range {min}-{max} for {grinder}")]
    GrindSettingOutOfRange { setting: i32, min: i32, max: i32, grinder: String },
    
    #[error("Coffee ratio {ratio:.1} outside acceptable range {min:.1}-{max:.1}")]
    CoffeeRatioInvalid { ratio: f32, min: f32, max: f32 },
    
    #[error("Quality rating must be in 0.5 increments between 1.0-10.0, got: {rating}")]
    InvalidQualityRating { rating: f32 },
    
    #[error("Coffee amount {amount}g outside valid range {min}-{max}g")]
    InvalidCoffeeAmount { amount: f32, min: f32, max: f32 },
    
    #[error("Water amount {amount}ml outside valid range {min}-{max}ml")]
    InvalidWaterAmount { amount: f32, min: f32, max: f32 },
    
    #[error("Brew time {time}s outside valid range {min}-{max}s")]
    InvalidBrewTime { time: i32, min: i32, max: i32 },
    
    #[error("Insufficient bean quantity: {available}g available, {required}g required")]
    InsufficientBeanQuantity { available: f32, required: f32 },
}
```

## Testing Strategy

### Unit Tests
- Test all service layer methods with mock data using `mockall`
- Test data model validation and serialization with `proptest`
- Test database operations with in-memory Postgres
- Use mutation testing with `cargo-mutants` to validate test quality

### Integration Tests
- Test complete workflows (add bean → add grinder → create session)
- Test data persistence and retrieval with real database
- Test filtering and search functionality end-to-end
- Test statistics calculation accuracy with various data sets

### Property-Based Testing
- Use `proptest` to generate random valid/invalid inputs for data models
- Test grind setting validation across different grinder ranges
- Validate rating bounds and coffee-to-water ratio calculations
- Generate edge cases for date handling and quantity tracking

### Test Quality Assurance
- Run `cargo-mutants` to ensure tests catch actual logic errors
- Maintain high mutation test score (>80%) for critical business logic
- Use `cargo-tarpaulin` for code coverage reporting
- Create fixtures with sample beans, grinders, and sessions for consistent testing

## Analytics and Optimization

### Grind Size Optimization Engine
The core analytics feature focuses on helping users find optimal grind settings based on historical data:

```rust
pub struct OptimizationEngine {
    service: BrewingService,
}

impl OptimizationEngine {
    // Analyze historical data to suggest optimal grind settings
    pub fn suggest_grind_setting(&self, bean_id: i32, method: BrewingMethod) -> Result<GrindSuggestion>;
    
    // Calculate correlations between grind size and brew quality
    pub fn analyze_grind_quality_correlation(&self) -> Result<CorrelationData>;
    
    // Identify brewing patterns and trends
    pub fn analyze_brewing_patterns(&self) -> Result<PatternAnalysis>;
    
    // Generate performance insights for equipment combinations
    pub fn analyze_equipment_performance(&self) -> Result<EquipmentAnalysis>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CorrelationData {
    pub grind_to_rating_correlation: f32,
    pub method_performance: Vec<MethodPerformance>,
    pub bean_grind_preferences: Vec<BeanGrindPreference>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternAnalysis {
    pub most_successful_combinations: Vec<SuccessfulCombination>,
    pub improvement_suggestions: Vec<ImprovementSuggestion>,
    pub consistency_metrics: ConsistencyMetrics,
}
```

### Statistical Analysis Features
- **Rating Trends**: Track brewing quality over time with trend analysis
- **Equipment Performance**: Compare grinder and bean combinations for optimal results
- **Brewing Consistency**: Measure variance in brewing parameters and outcomes
- **Success Pattern Recognition**: Identify parameter combinations that consistently produce high ratings

### Data-Driven Recommendations
The system will provide intelligent suggestions based on:
1. Historical rating data for specific bean-grinder-method combinations
2. Statistical analysis of grind setting effectiveness
3. Correlation analysis between brewing parameters and quality ratings
4. Pattern recognition for successful brewing workflows

## Configuration

### Application Configuration
```rust
#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    database_url: String,
    max_connections: u32,
    default_brewing_method: BrewingMethod,
    auto_update_bean_quantity: bool,
    date_format: String,
    theme: Theme,
}

#[derive(Debug, Serialize, Deserialize)]
enum Theme {
    Light,
    Dark,
    Auto,
}
```

Configuration will be stored in `~/.config/coffee-tracker/config.toml` with sensible defaults. The database URL will default to a local Postgres instance: `postgresql://localhost/coffee_tracker` with automatic database creation and migration on first run.

### Dioxus Architecture
The Dioxus frontend will feature:
- **Component-based Architecture** with reusable UI components
- **Reactive State Management** using Dioxus hooks and shared state
- **Client-side Routing** for navigation between pages
- **Responsive Design** using CSS-in-Rust or external stylesheets
- **Future Desktop Support** using the same codebase with `dioxus-desktop`

### Routing
```rust
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Dashboard {},
    #[route("/brew")]
    NewBrew {},
    #[route("/sessions")]
    Sessions {},
    #[route("/sessions/:id")]
    SessionDetail { id: u32 },
    #[route("/equipment")]
    Equipment {},
    #[route("/stats")]
    Statistics {},
}
```