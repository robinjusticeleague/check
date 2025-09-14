# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Development Commands

### Primary Commands (use Just task runner)

```bash
# Install required development tools
just install-tools

# Format code (Rust + TOML)
just format

# Run all tests
just test

# Run linting
just lint

# Check if ready for CI (comprehensive check)
just ready

# Generate all code-generated files
just gen-all
```

### Cargo Commands

```bash
# Run Check CLI in development mode
cargo check-cli-dev --help
cargo check-cli-dev check --write

# Run tests for specific crate
cargo test -p check_html_formatter

# Build in release mode
cargo build --release
```

### Rule Development

```bash
# Create new lint rules (name must be camelCase)
just new-js-lintrule ruleName
just new-css-lintrule ruleName
just new-json-lintrule ruleName
just new-graphql-lintrule ruleName

# Test specific lint rule
just test-lintrule ruleName

# Move rule from nursery to stable group
just move-rule stable ruleName
```

## Architecture Overview

**Check** is a Rust-based monorepo implementing a high-performance web development toolchain. The architecture follows a modular, language-agnostic design.

### Core Components

- **check_service**: Central workspace service providing unified APIs
- **check_rowan**: Lossless syntax tree foundation (preserves all source info)
- **check_formatter**: Pretty-printing infrastructure using intermediate representation
- **check_analyze**: Rule-based analysis framework with visitor patterns

### Language Support Pattern

Each language follows consistent crate structure:
```
check_{lang}_syntax     # AST definitions
check_{lang}_parser     # Parser implementation
check_{lang}_formatter  # Formatting logic
check_{lang}_analyze    # Linting rules
check_{lang}_factory    # AST construction
check_{lang}_semantic   # Semantic analysis (where applicable)
```

Languages: JavaScript/TypeScript, CSS, JSON, HTML, GraphQL, Grit

### Key Architectural Patterns

1. **Grammar-Driven**: Uses `.ungram` files for AST generation
2. **Code Generation**: Extensive use of `xtask/codegen` for consistency
3. **Incremental**: Supports efficient re-parsing and re-analysis
4. **Transport-Agnostic**: Service layer works with CLI, LSP, daemon
5. **Capability-Based**: File handlers declare their supported operations

### File Locations

- Core crates: `/crates/check_*/`
- Grammar definitions: `/xtask/codegen/*.ungram`
- Test snapshots: `tests/specs/` in each crate
- Configuration: `check_configuration/src/`

### Testing Strategy

- **Spec Tests**: Snapshot testing with `.snap` files (use `cargo insta`)
- **Quick Tests**: `just test-quick package` for rapid iteration
- **Prettier Compatibility**: Automated comparison tests
- **Fuzz Testing**: In `/fuzz/` directory

### Development Workflow

1. Make changes to source code
2. Run `just format` to format code
3. Run `just gen-all` if modifying grammars or adding rules
4. Run `just test` to verify changes
5. Use `just ready` before submitting PR (runs all CI checks)

### Working with Current Directory

You are currently in `/crates/check_html_formatter/` - the HTML formatter implementation. This crate:

- Implements HTML/Astro/Svelte formatting
- Follows the standard formatter architecture using `check_formatter` IR
- Has test files in `tests/` directory
- Benchmark in `benches/html_formatter.rs`

### Code Generation

When adding new syntax nodes or rules:
- Modify appropriate `.ungram` file in `xtask/codegen/`
- Run `just gen-all` to regenerate code
- Formatters, parsers, and AST definitions are auto-generated

### Performance Notes

- Uses arena allocation for memory efficiency
- Implements incremental parsing/analysis
- Zero-copy string handling where possible
- All formatters use same IR for consistency

### Additional Instructions to know how to work inside the project

- General contribution instructions @CONTRIBUTING.md
