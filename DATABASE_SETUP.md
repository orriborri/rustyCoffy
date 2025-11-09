# Database Setup Guide

## Quick Start with Devbox

The easiest way to get started is using devbox, which provides an isolated development environment with PostgreSQL.

### Prerequisites

Install devbox:
```bash
curl -fsSL https://get.jetpack.io/devbox | bash
```

### Setup

1. **Run the setup script:**
   ```bash
   ./setup-dev.sh
   ```

2. **Enter the development shell:**
   ```bash
   devbox shell
   ```

3. **Start developing:**
   ```bash
   cargo run
   ```

## Manual Setup (Alternative)

If you prefer to use your own PostgreSQL installation:

### 1. Install PostgreSQL and Diesel CLI

```bash
# Install PostgreSQL (varies by OS)
# Ubuntu/Debian:
sudo apt-get install postgresql postgresql-contrib

# macOS:
brew install postgresql

# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres
```

### 2. Create Database

```bash
# Start PostgreSQL service
sudo systemctl start postgresql  # Linux
brew services start postgresql   # macOS

# Create database
createdb coffee_tracker

# Set environment variable
export DATABASE_URL=postgresql://username:password@localhost/coffee_tracker
```

### 3. Run Migrations

```bash
diesel migration run
```

## Database Commands

### Using Devbox (Recommended)

```bash
# Start database
devbox run start-db

# Stop database  
devbox run stop-db

# Reset database (destroys all data)
devbox run reset-db

# Run migrations
devbox run run-migrations

# Verify schema
devbox run verify-schema

# Test constraints
devbox run test-constraints
```

### Using Diesel CLI Directly

```bash
# Run migrations
diesel migration run

# Rollback last migration
diesel migration revert

# Generate new migration
diesel migration generate migration_name

# Print current schema
diesel print-schema > src/schema.rs
```

## Database Connection Details

- **Host:** localhost
- **Port:** 5432
- **Database:** coffee_tracker
- **User:** postgres (devbox setup)
- **URL:** postgresql://postgres@localhost:5432/coffee_tracker

## Schema Verification

After setup, verify the schema is correct:

```bash
# Check tables exist
psql -d coffee_tracker -c "\dt"

# Verify constraints
devbox run verify-schema

# Test constraint enforcement
devbox run test-constraints
```

## Troubleshooting

### Database Connection Issues

1. **Check if PostgreSQL is running:**
   ```bash
   devbox run start-db
   # or manually: pg_ctl status -D $PGDATA
   ```

2. **Check database exists:**
   ```bash
   psql -l | grep coffee_tracker
   ```

3. **Recreate database:**
   ```bash
   devbox run reset-db
   ```

### Migration Issues

1. **Check migration status:**
   ```bash
   diesel migration list
   ```

2. **Rollback and retry:**
   ```bash
   diesel migration revert
   diesel migration run
   ```

### Permission Issues

If you get permission errors, ensure PostgreSQL is configured for local connections:

```bash
# Edit pg_hba.conf to allow local connections
# Add line: local all all trust
```

## Development Workflow

1. **Start development session:**
   ```bash
   devbox shell
   devbox run start-db
   ```

2. **Make schema changes:**
   ```bash
   diesel migration generate add_new_feature
   # Edit the generated up.sql and down.sql files
   diesel migration run
   ```

3. **Update Rust schema:**
   ```bash
   diesel print-schema > src/schema.rs
   ```

4. **Test changes:**
   ```bash
   cargo test
   devbox run test-constraints
   ```

5. **Stop when done:**
   ```bash
   devbox run stop-db
   exit  # Leave devbox shell
   ```

## Schema Files

- `migrations/` - Database migration files
- `src/schema.rs` - Generated Diesel schema
- `verify_schema.sql` - Schema verification script
- `test_constraints.sql` - Constraint testing script
- `diesel.toml` - Diesel configuration