#!/bin/bash

# Coffee Brewing Tracker - Development Setup Script

set -e

echo "🚀 Setting up Coffee Brewing Tracker development environment..."

# Check if devbox is installed
if ! command -v devbox &> /dev/null; then
    echo "❌ Devbox is not installed. Please install it first:"
    echo "   curl -fsSL https://get.jetpack.io/devbox | bash"
    exit 1
fi

echo "📦 Installing development dependencies..."
devbox install

echo "🗄️  Setting up PostgreSQL and databases..."
devbox run setup-dev

echo "🧪 Running core tests (no database required)..."
devbox run test-core

echo "🗃️  Running database tests..."
devbox run test-db

echo ""
echo "🎉 Development environment setup complete!"
echo ""
echo "📋 Development Status:"
devbox run dev-status

echo ""
echo "🚀 Quick Start Commands:"
echo "  devbox shell              - Enter development shell"
echo "  devbox run dev-status     - Show current status"
echo "  devbox run test-core      - Run core tests (fast, no DB)"
echo "  devbox run test-db        - Run database tests"
echo "  devbox run test-all       - Run all tests"
echo "  devbox run build-core     - Build core library"
echo "  devbox run build-full     - Build with database features"
echo ""
echo "🗄️  Database Commands:"
echo "  devbox run start-db       - Start PostgreSQL"
echo "  devbox run stop-db        - Stop PostgreSQL"
echo "  devbox run reset-db       - Reset database (⚠️  destroys data)"
echo "  devbox run run-migrations - Apply database migrations"
echo ""
echo "🔗 Database Connections:"
echo "  Main DB:  postgresql://postgres@localhost:5432/coffee_tracker"
echo "  Test DB:  postgresql://postgres@localhost:5432/coffee_tracker_test"
echo ""
echo "✨ Ready to develop! Try:"
echo "  devbox shell"
echo "  cargo test --lib --no-default-features  # Fast core tests"
echo "  cargo test --lib --features postgres-tests  # Full database tests"