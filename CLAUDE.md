# Weather Forecast Rust Project Guidelines

## Build & Run Commands
- Build: `cargo build`
- Run: `cargo run`
- Test all: `cargo test`
- Test single: `cargo test test_name` or `cargo test path::to::test`
- Format code: `cargo fmt`
- Lint code: `cargo clippy`
- Check without building: `cargo check`
- Fix linting issues: `cargo clippy --fix`
- Apply migrations: `cargo run -p migration`

## Code Style Guidelines
- Use `rustfmt` for consistent formatting
- Follow the Rust standard naming conventions (snake_case for functions/variables, CamelCase for types)
- Organize imports: std first, then external crates, then internal modules
- Error handling: Use `thiserror` for error definitions, proper propagation with `?` operator
- Prefer strong typing with structs and enums over primitive types
- Use logging through the `log` crate with appropriate levels (debug, info, warn, error)
- Use async/await properly with correct error handling
- Include comprehensive docstrings for public APIs
- Test public functions and edge cases