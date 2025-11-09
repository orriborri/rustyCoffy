# Project Structure

## Current Organization
```
.
├── .kiro/
│   └── steering/          # AI assistant steering rules
├── .vscode/
│   └── settings.json      # VSCode configuration
```

## Recommended Structure Patterns
As the project grows, follow these organizational principles:

### Source Code
```
src/                       # Main source code
├── components/            # Reusable components
├── utils/                 # Utility functions
├── types/                 # Type definitions
└── tests/                 # Unit tests
```

### Configuration
```
config/                    # Configuration files
docs/                      # Documentation
scripts/                   # Build and utility scripts
```

### Assets and Resources
```
assets/                    # Static assets
public/                    # Public files
resources/                 # Resource files
```

## Naming Conventions
- Use kebab-case for file and folder names
- Use PascalCase for component names
- Use camelCase for function and variable names
- Keep names descriptive but concise

## File Organization Rules
- Group related functionality together
- Keep configuration files at the root level
- Separate source code from build artifacts
- Use clear, descriptive folder names
- Maintain consistent depth levels (avoid deeply nested structures)

## Special Directories
- `.kiro/` - Kiro AI assistant configuration (do not modify manually)
- `.vscode/` - VSCode workspace settings
- `node_modules/`, `venv/`, `target/` - Generated dependencies (add to .gitignore)