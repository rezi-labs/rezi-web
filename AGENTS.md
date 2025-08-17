# Agent Guidelines for Rezi Web

## Build/Test Commands
- **Build**: `cargo build`
- **Run**: `just run` (starts database and server)
- **Test**: `cargo test` or `just test`
- **Single test**: `cargo test test_name`
- **Lint**: `just lint` (runs `cargo fmt --check` and `cargo clippy`)
- **Format**: `just fmt` (runs `cargo fmt` and `cargo fix`)
- **Verify**: `just verify` (runs lint and test)
- **Watch**: `just watch` (auto-reload on changes)

## Code Style
- Use `snake_case` for functions, variables, modules
- Use `PascalCase` for structs, enums, traits
- Import grouping: std → external crates → local modules
- Use `web::Data<T>` for dependency injection in handlers
- Database operations use `DBClient = Arc<Mutex<DB>>`
- Error handling: prefer `Result<T>` with proper error propagation
- Use `maud::html!` for HTML templating
- Async handlers return `Result<Markup>` or `Result<HttpResponse>`
- Log with `log::info!`, `log::error!` etc.
- Use Actix-web attributes: `#[get("/path")]`, `#[post("/path")]` etc.

## Libraries

- use htmx where possible
- use daisy ui first and not tailwind where possible, see daisyui.llm.txt for documentation
- use tailwind second and not css third.
- use css fourth and not js fifth.

## Styling
- always use swiss design principles