# Agent Guidelines for kickstart-napi-rs

This is a NAPI-RS project that creates Node.js native addons using Rust. The codebase uses TypeScript for tests and JavaScript bindings, with Rust for the native implementation.

## Build Commands

```bash
# Build native addon (release)
bun run build

# Build native addon (debug)
bun run build:debug

# Format all code (TypeScript, Rust, TOML)
bun run format

# Lint TypeScript/JavaScript
bun run lint

# Run all tests
bun run test

# Run single test file
bunx vitest run tests/plus-100.test.ts

# Run benchmarks
bun run bench

# Generate changelog
bun run changelog
```

## Code Style Guidelines

### TypeScript/JavaScript

- **Formatter**: Use `oxfmt` with single quotes
- **Linter**: Use `oxlint` with strict TypeScript rules
- **Import sorting**: Enabled with experimental sort, grouped by type (external, internal, parent, sibling)
- **Strict mode**: Enabled in tsconfig.json
- **Quotes**: Single quotes only
- **Console**: Avoid `console.log` (enforced by lint rule `no-console`)

### Rust

- **Formatter**: Use `rustfmt` with 2-space indentation
- **Linter**: Use `clippy` with `#![deny(clippy::all)]` at top of lib.rs
- **Naming**: Use snake_case for functions/variables, PascalCase for types
- **Attributes**: Place each attribute on its own line before functions
- **Style**: Align struct fields with spaces (see Cargo.toml for examples)

### Test Structure

```typescript
// tests/example.test.ts
import { describe, it, expect } from 'vitest';
import { functionName } from '../index.js';

describe('feature name', () => {
  it('does something specific', () => {
    expect(functionName(input)).toBe(expected);
  });
});
```

### Benchmark Structure

```typescript
// benchmark/feature.bench.ts
import { bench, describe } from 'vitest';
import { rustFunction } from '../index.js';

describe('comparison name', () => {
  bench('Rust implementation', () => {
    rustFunction(input);
  });

  bench('JavaScript implementation', () => {
    jsFunction(input);
  });
});
```

## Project Structure

```
├── src/lib.rs              # Rust source code with #[napi] exports
├── index.js                # Generated JavaScript bindings (DO NOT EDIT)
├── index.d.ts              # Generated TypeScript definitions (DO NOT EDIT)
├── tests/                  # Vitest test files (*.test.ts)
├── benchmark/              # Vitest benchmark files (*.bench.ts)
├── Cargo.toml              # Rust package configuration
├── package.json            # Node.js package configuration
├── vitest.config.ts        # Vitest configuration
├── .oxfmtrc.jsonc          # Oxfmt formatter config
├── .oxlintrc.json          # Oxlint configuration
└── rustfmt.toml            # Rust formatter config (2 spaces)
```

## NAPI-RS Conventions

- Export Rust functions with `#[napi]` attribute
- Use snake_case in Rust, functions become camelCase in JavaScript
- Primitive types auto-convert (u32 -> number, String -> string)
- Complex types need explicit NAPI bindings
- Always build before testing: `bun run build`

## Pre-commit Hooks

Husky runs lint-staged which:

1. Runs `oxlint --fix` on JS/TS files
2. Runs `oxfmt` on JS/TS/JSON/JSONC/MD/YML/YAML files
3. Runs `taplo format` on TOML files

## Common Tasks

- Add new Rust function: Edit `src/lib.rs`, add `#[napi]` attribute, rebuild
- Add test: Create `tests/feature.test.ts`, import from `../index.js`
- Run single test: `bunx vitest run tests/specific.test.ts`
- Debug build: Use `bun run build:debug` for faster compilation during development
