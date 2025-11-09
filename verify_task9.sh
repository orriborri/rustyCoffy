#!/bin/bash

echo "=== Task 9 Verification: Multi-step Brewing Session Creation ==="
echo ""

echo "1. Checking if NewBrew page exists..."
if [ -f "src/ui/pages/new_brew.rs" ]; then
    echo "✓ NewBrew page file exists"
else
    echo "✗ NewBrew page file not found"
    exit 1
fi

echo ""
echo "2. Checking for required components..."

# Check for BeanSelector component
if grep -q "fn BeanSelector" src/ui/pages/new_brew.rs; then
    echo "✓ BeanSelector component found"
else
    echo "✗ BeanSelector component not found"
fi

# Check for GrinderSelector component
if grep -q "fn GrinderSelector" src/ui/pages/new_brew.rs; then
    echo "✓ GrinderSelector component found"
else
    echo "✗ GrinderSelector component not found"
fi

# Check for BrewingMethodSelector component
if grep -q "fn BrewingMethodSelector" src/ui/pages/new_brew.rs; then
    echo "✓ BrewingMethodSelector component found"
else
    echo "✗ BrewingMethodSelector component not found"
fi

# Check for RatingInput component
if grep -q "fn RatingInput" src/ui/pages/new_brew.rs; then
    echo "✓ RatingInput component found"
else
    echo "✗ RatingInput component not found"
fi

echo ""
echo "3. Checking for key features..."

# Check for search functionality in BeanSelector
if grep -q "search_query" src/ui/pages/new_brew.rs; then
    echo "✓ Bean search functionality found"
else
    echo "✗ Bean search functionality not found"
fi

# Check for grind setting validation
if grep -q "validate_grind_setting" src/ui/pages/new_brew.rs; then
    echo "✓ Grind setting validation found"
else
    echo "✗ Grind setting validation not found"
fi

# Check for coffee-to-water ratio calculation
if grep -q "calculate_ratio" src/ui/pages/new_brew.rs; then
    echo "✓ Coffee-to-water ratio calculation found"
else
    echo "✗ Coffee-to-water ratio calculation not found"
fi

# Check for rating validation (0.5 increments)
if grep -q "validate_rating" src/ui/pages/new_brew.rs; then
    echo "✓ Rating validation found"
else
    echo "✗ Rating validation not found"
fi

# Check for session saving
if grep -q "create_session" src/ui/pages/new_brew.rs; then
    echo "✓ Session saving functionality found"
else
    echo "✗ Session saving functionality not found"
fi

# Check for validation error handling
if grep -q "validation_errors" src/ui/pages/new_brew.rs; then
    echo "✓ Validation error handling found"
else
    echo "✗ Validation error handling not found"
fi

echo ""
echo "=== Verification Complete ==="
