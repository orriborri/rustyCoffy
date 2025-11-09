---
inclusion: always
---

# Coffee Brewing Tracker - Product Rules

## Domain Model Constraints (CRITICAL)

### Core Data Model
```rust
struct BrewingSession {
    grinder_id: i32,        // FK to grinders table
    bean_id: i32,           // FK to beans table  
    method: BrewingMethod,  // Enum: PourOver | FrenchPress | Espresso | AeroPress | Chemex
    grind_setting: i32,     // Range: grinder.min_setting..=grinder.max_setting
    coffee_amount: f32,     // Range: 10.0..=100.0 grams
    water_amount: f32,      // Range: 150.0..=1700.0 ml
    brew_time: i32,         // Range: 30..=480 seconds
    quality_rating: f32,    // Values: 1.0, 1.5, 2.0, ..., 9.5, 10.0 ONLY
}
```

### Non-Negotiable Validation Rules
1. **Coffee Ratio**: `water_amount / coffee_amount` must be between 15.0 and 17.0 (inclusive)
2. **Quality Rating**: Must be in 0.5 increments from 1.0 to 10.0 - reject any other values
3. **Grind Setting**: Must be within the specific grinder's min/max range from database
4. **Foreign Key Integrity**: Always validate grinder_id and bean_id exist before saving
5. **Domain Layer Validation**: All validation logic belongs in domain models, never in UI

### Standard Error Types
```rust
#[derive(thiserror::Error, Debug)]
pub enum ValidationError {
    #[error("Grind setting {setting} outside range {min}-{max} for {grinder}")]
    GrindSettingOutOfRange { setting: i32, min: i32, max: i32, grinder: String },
    #[error("Coffee ratio {ratio:.1} outside acceptable range 15.0-17.0")]
    CoffeeRatioInvalid { ratio: f32 },
    #[error("Quality rating must be in 0.5 increments between 1.0-10.0")]
    InvalidQualityRating { rating: f32 },
}
```

## Architecture Patterns (MANDATORY)

### Clean Architecture Layers
- **Domain Layer**: Pure Rust structs with validation logic - zero external dependencies
- **Service Layer**: Repository pattern using Diesel ORM with r2d2 connection pooling
- **UI Layer**: Dioxus components for presentation only - no business logic

### Dioxus Component Template
```rust
#[derive(Props, PartialEq)]
struct ComponentProps {
    // Always include event handlers for data flow
    on_submit: EventHandler<DataType>,
    // Props for data down, events for actions up
}

#[component]
fn ComponentName(props: ComponentProps) -> Element {
    let mut local_state = use_signal(|| default_value);
    // Keep components focused on single responsibility
    rsx! { /* JSX-like syntax */ }
}
```

### Naming Conventions (STRICT)
- **Components**: PascalCase (`BrewingSessionForm`, `GrinderSelector`)
- **Props Structs**: Always `#[derive(Props, PartialEq)]`
- **State Management**: `use_signal()` for local state, props for parent data
- **Event Handlers**: `EventHandler<T>` for all user interactions

## User Experience Rules

### Form Workflow (FIXED SEQUENCE)
1. Bean Selection → 2. Grinder Selection → 3. Grind Setting → 4. Brew Parameters → 5. Quality Rating

### Required Default Values
```rust
const DEFAULTS: BrewingDefaults = BrewingDefaults {
    coffee_amount: 20.0,    // grams
    water_amount: 320.0,    // ml (maintains 16:1 ratio)
    brew_time: 240,         // seconds (4 minutes)
    method: BrewingMethod::PourOver,
};
```

### Validation UX Patterns
- **Timing**: Validate on blur events, not on every keystroke
- **Visual Feedback**: Red borders for validation errors, green for quality ratings ≥ 7.0
- **Real-time Display**: Show coffee-to-water ratio calculation as user types
- **Error Messages**: Display specific validation errors with current and expected values

## Development Workflow (REQUIRED ORDER)

### Implementation Sequence
1. **Database Schema**: Create Diesel migrations before any code
2. **Domain Models**: Implement all validation rules in pure Rust structs
3. **Repository Layer**: Database access with connection pooling
4. **Service Layer**: Business logic orchestration and error handling
5. **UI Components**: Dioxus components with form validation

### Testing Strategy (COMPREHENSIVE)
- **Unit Tests**: Every validation rule must have passing and failing test cases
- **Property Tests**: Use `proptest` for numeric range validation (coffee ratios, ratings)
- **Integration Tests**: Complete user workflows from form submission to database
- **Mutation Tests**: Run `cargo mutants` to verify test quality and coverage
- **Component Tests**: Use `mockall` to isolate UI components from dependencies

### Quality Assurance Gates
- All domain validation rules require both positive and negative test cases
- Coffee ratio calculations must be property-tested with edge cases
- Form submission workflows must be integration-tested end-to-end
- Mutation testing must pass with high confidence scores

## AI Assistant Guidelines

### When Writing Code
- Always implement domain validation first, UI validation second
- Use exact error message formats from ValidationError enum
- Follow the fixed form workflow sequence in UI components
- Apply default values consistently across all forms
- Validate foreign key relationships before database operations

### When Debugging
- Check domain layer validation before investigating UI issues
- Verify coffee ratio calculations match the 15.0-17.0 range requirement
- Ensure quality ratings are exactly in 0.5 increments
- Confirm grind settings are within grinder-specific ranges