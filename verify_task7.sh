#!/bin/bash

echo "=== Task 7 Verification: Dioxus Application Structure ==="
echo ""

echo "1. Checking UI module structure..."
ls -la src/ui/ 2>/dev/null && echo "✓ UI module exists" || echo "✗ UI module missing"
echo ""

echo "2. Checking components..."
ls -la src/ui/components/ 2>/dev/null && echo "✓ Components directory exists" || echo "✗ Components missing"
echo ""

echo "3. Checking pages..."
ls -la src/ui/pages/ 2>/dev/null && echo "✓ Pages directory exists" || echo "✗ Pages missing"
echo ""

echo "4. Checking configuration files..."
[ -f "Dioxus.toml" ] && echo "✓ Dioxus.toml exists" || echo "✗ Dioxus.toml missing"
[ -f "index.html" ] && echo "✓ index.html exists" || echo "✗ index.html missing"
[ -f "assets/style.css" ] && echo "✓ CSS file exists" || echo "✗ CSS file missing"
echo ""

echo "5. Checking dependencies..."
grep -q "dioxus-router" Cargo.toml && echo "✓ dioxus-router dependency added" || echo "✗ dioxus-router missing"
echo ""

echo "6. Building library..."
cargo build --lib 2>&1 | tail -3
echo ""

echo "7. Checking route definitions..."
grep -A 20 "enum Route" src/ui/app.rs | head -15
echo ""

echo "=== Verification Complete ==="
