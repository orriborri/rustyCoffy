# Coffee Brewing Tracker - Devbox Development Guide

## 🚀 Quick Start

### 1. Initial Setup
```bash
# Run the setup script (one-time setup)
./setup-dev.sh
```

### 2. Daily Development
```bash
# Enter the development environment
devbox shell

# Check status
devbox run dev-status

# Run tests
devbox run test-core    # Fast core tests (no database)
devbox run test-db      # Database tests
devbox run test-all     # All tests
```

## 📋 Available Commands

### Development Workflow
```bash
devbox shell              # Enter development shell with all tools
devbox run dev-status     # Show current development status
devbox run setup-dev      # Full development environment setup
```

### Testing Commands
```bash
devbox run test-core      # Core tests (no database) - FAST ⚡
devbox run test-db        # Database integration tests
devbox run test-all       # Run both core and database tests
```

### Build Commands
```bash
devbox run build-core     # Build core library (no database features)
devbox run build-full     # Build with all database features
```

### Database Management
```bash
devbox run start-db       # Start PostgreSQL server
devbox run stop-db        # Stop PostgreSQL server
devbox run reset-db       # Reset database (⚠️ destroys all data)
devbox run run-migrations # Apply database migrations
devbox run verify-schema  # Verify database schema
devbox run test-constraints # Test database constraints
```

## 🏗️ Architecture Overview

### Core Development (No Database)
```bash
# Fast development cycle - no database required
devbox run test-core
cargo test --lib --no-default-features
cargo check --lib --no-default-features
```

**What's Available:**
- ✅ Domain models and business logic
- ✅ Validation rules (coffee ratios, quality ratings)
- ✅ Property-based testing with proptest
- ✅ Enum handling and type safety
- ✅ Fast test execution (21 tests in ~0.3s)

### Full Database Development
```bash
# Full development with database features
devbox run start-db
devbox run test-db
cargo test --lib --features postgres-tests
```

**What's Available:**
- ✅ Complete CRUD operations
- ✅ Connection pooling with r2d2
- ✅ Advanced filtering and search
- ✅ Integration tests
- ✅ Inventory management

## 🗄️ Database Configuration

### Databases Created
- **coffee_tracker** - Main development database
- **coffee_tracker_test** - Testing database

### Connection Details
```bash
# Main database
DATABASE_URL=postgresql://postgres@localhost:5432/coffee_tracker

# Test database  
TEST_DATABASE_URL=postgresql://postgres@localhost:5432/coffee_tracker_test
```

### Environment Variables
```bash
PGDATA=$PWD/.devbox/virtenv/postgresql/data
PGHOST=localhost
PGPORT=5432
PGUSER=postgres
```

## 🧪 Testing Strategy

### Fast Development Cycle
```bash
# Core tests only (no database setup required)
devbox run test-core
# Result: 21 tests pass in ~0.3 seconds ⚡
```

### Full Integration Testing
```bash
# Database tests (requires PostgreSQL)
devbox run start-db
devbox run test-db
# Result: Full CRUD and integration tests
```

## 🔧 Troubleshooting

### PostgreSQL Issues
```bash
# Check PostgreSQL status
devbox run dev-status

# Restart PostgreSQL
devbox run stop-db
devbox run start-db

# Reset everything (nuclear option)
devbox run reset-db
```

### Compilation Issues
```bash
# Core compilation (should always work)
cargo check --lib --no-default-features

# Database compilation (requires PostgreSQL)
cargo check --lib --features database
```

### Test Issues
```bash
# Run specific test
cargo test validate_coffee_ratio --lib --no-default-features

# Run with output
cargo test --lib --no-default-features -- --nocapture
```

## 📦 Package Management

### Included Packages
- **postgresql@15** - PostgreSQL database server
- **diesel-cli** - Database migration tool
- **pkg-config** - Build configuration tool
- **openssl** - SSL/TLS library

### Adding New Packages
```bash
# Add a new package to devbox
devbox add <package-name>

# Update devbox.json manually and install
devbox install
```

## 🎯 Development Workflows

### Feature Development
```bash
# 1. Start development environment
devbox shell

# 2. Run core tests frequently (fast feedback)
devbox run test-core

# 3. When ready, test database integration
devbox run start-db
devbox run test-db

# 4. Build and verify
devbox run build-full
```

### Bug Fixing
```bash
# 1. Reproduce with core tests (fast)
cargo test <specific-test> --lib --no-default-features

# 2. Fix and verify
devbox run test-core

# 3. Test database integration if needed
devbox run test-db
```

### Database Schema Changes
```bash
# 1. Create migration
diesel migration generate <migration-name>

# 2. Edit migration files
# migrations/*/up.sql and down.sql

# 3. Apply migration
devbox run run-migrations

# 4. Verify schema
devbox run verify-schema

# 5. Test constraints
devbox run test-constraints
```

## 🚀 Production Readiness

The development environment supports both:
- **Core Development**: Fast iteration without database overhead
- **Full Integration**: Complete database testing and validation

This dual approach ensures:
- ⚡ Fast development cycles
- 🔒 Robust integration testing
- 🏗️ Clean architecture separation
- 🎯 Production-ready code quality

---

**Happy coding! ☕** The Coffee Brewing Tracker development environment is ready for productive development.