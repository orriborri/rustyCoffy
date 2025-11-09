# Technology Stack

## Coffee Brewing Tracker - Rust/Dioxus Application

### Core Technologies
- **Language**: Rust (latest stable)
- **Frontend Framework**: Dioxus for reactive UI components
- **Web Target**: `dioxus-web` for browser deployment
- **Desktop Target**: `dioxus-desktop` for native applications (future)
- **Database**: Postgres with Diesel ORM for type-safe operations
- **Connection Pooling**: r2d2 for database connection management

### Key Dependencies
```toml
[dependencies]
dioxus = "0.4"
dioxus-web = "0.4"
diesel = { version = "2.1", features = ["Postgres", "chrono", "r2d2"] }
r2d2 = "0.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
diesel_migrations = "2.1"
proptest = "1.4"           # Property-based testing
mockall = "0.12"           # Mock generation for testing
```

### Testing Tools
- **Mutation Testing**: `cargo-mutants` for test quality assessment
- **Property Testing**: `proptest` for generating test cases
- **Mocking**: `mockall` for creating test doubles
- **Coverage**: `cargo-tarpaulin` for code coverage analysis

### Development Environment
- **IDE**: VSCode with Rust Analyzer extension
- **AI Tools**: Kiro agent for development assistance
- **Platform**: Cross-platform Rust development
- **Database Tools**: Diesel CLI for migrations and schema management

### Project Commands
```bash
# Development
cargo run                    # Run the application
cargo run --bin setup        # Initialize database and run migrations
dx serve                     # Serve Dioxus web app (requires dx CLI)

# Database Management
diesel setup                 # Initialize database
diesel migration generate    # Create new migration
diesel migration run         # Apply pending migrations
diesel print-schema          # Generate schema.rs

# Testing
cargo test                   # Run all tests
cargo test --lib             # Run unit tests only
cargo test --test integration # Run integration tests

# Advanced Testing
cargo install cargo-mutants  # Install mutation testing tool
cargo mutants                # Run mutation testing
cargo tarpaulin              # Generate code coverage report
cargo test -- --nocapture   # Run tests with output

# Building
cargo build                  # Debug build
cargo build --release        # Production build
dx build --release           # Build Dioxus web bundle

# Code Quality
cargo fmt                    # Format code
cargo clippy                 # Lint code
cargo check                  # Fast compilation check
```

### Database Configuration
- **Primary**: Postgres for local storage and portability
- **Location**: `~/.local/share/coffee-tracker/database.db`
- **Migrations**: Managed through Diesel CLI
- **Schema**: Auto-generated in `src/schema.rs`

### Architecture Patterns
- **Clean Architecture**: Separation of data, business logic, and UI layers
- **Repository Pattern**: Database abstraction through service layer
- **Component-Based UI**: Reusable Dioxus components
- **Type-Safe Database**: Diesel ORM with compile-time query validation

### Development Workflow
1. **Database First**: Create migrations before implementing models
2. **Test-Driven**: Write tests for business logic before implementation
3. **Mutation Testing**: Use `cargo-mutants` to validate test quality and coverage
4. **Property-Based Testing**: Use `proptest` for comprehensive input validation
5. **Component Isolation**: Build UI components independently with mocked dependencies
6. **Incremental Development**: Small, testable changes with continuous quality assessment

### Configuration Files
- `Cargo.toml` - Rust project configuration and dependencies
- `diesel.toml` - Database configuration and migration settings
- `Dioxus.toml` - Dioxus build and serve configuration
- `.vscode/settings.json` - VSCode Rust development settings

### Best Practices
- Use `anyhow` for error handling in applications, `thiserror` for libraries
- Implement `Debug`, `Clone`, `Serialize`, `Deserialize` for data models
- Use Diesel's type system to prevent SQL injection and runtime errors
- Keep components small and focused on single responsibilities
- Write integration tests for complete user workflows
- Use Rust's ownership system to prevent data races and memory issues

### Testing Quality Assurance
- **Mutation Testing**: Run `cargo mutants` regularly to ensure tests catch actual bugs
- **Property-Based Testing**: Use `proptest` for data validation and edge case discovery
- **Test Coverage**: Maintain high coverage with meaningful assertions
- **Mock Isolation**: Use `mockall` to isolate units under test from dependencies
- **Continuous Testing**: Integrate mutation testing into CI/CD pipeline for quality gates