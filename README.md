# Axum Template

A batteries-included template for building REST APIs with [Axum](https://github.com/tokio-rs/axum). Ships with structured error handling, environment configuration, request logging, rate limiting, and Swagger/OpenAPI documentation via [utoipa](https://github.com/juhaku/utoipa).

## Quick Start

### 1. Clone & rename

```bash
git clone https://github.com/VinukaThejana/axum_tmpl.git my_project
cd my_project
```

### 2. Replace all references to `axum_tmpl`

Update the project name in the following places:

- `Cargo.toml` → `name = "my_project"`
- Every `use axum_tmpl::` import in `src/`
- The crate name in `src/lib.rs` (if referenced)

A quick way to do this:

```bash
# macOS / BSD / GNU sed
find . -type f -name '*.rs' -o -name '*.toml' | xargs sed -i '' 's/axum_tmpl/my_project/g'
```

### 3. Remove the template remote & amend the initial commit

```bash
git remote remove origin
```

Make whatever changes you need (add routes, schemas, middleware, etc.), then squash everything into a clean initial commit:

```bash
git add -A
git commit --amend -m "initial commit"
```

### 4. Push to your own repository

```bash
git remote add origin https://github.com/<you>/<your_repo>.git
git push -u origin main --force
```

### 5. Set up environment

```bash
cp .env.example .env
# Edit .env with your values
```

### 6. Run

```bash
cargo run
```

The server starts at `http://localhost:8080`. Swagger UI is available at `http://localhost:8080/swagger-ui`.

## Project Structure

```
.
├── Cargo.toml
├── src
│   ├── main.rs         # Entry point, router assembly, middleware stack
│   ├── lib.rs          # Library root (re-exports modules)
│   ├── doc.rs          # OpenAPI / Swagger configuration (utoipa)
│   ├── error.rs        # AppError enum + IntoResponse impl
│   ├── config
│   │   ├── env.rs      # Environment variable loading (envy)
│   │   ├── log.rs      # Logging setup (fern)
│   │   └── state.rs    # Shared application state
│   ├── handler         # Route handlers
│   ├── middleware       # Custom middleware (auth, etc.)
│   ├── routes          # Route grouping / nesting
│   ├── schemas         # Request/response types (serde + utoipa ToSchema)
│   └── util            # Helpers (rate limiter config, shutdown, etc.)
└── .env.example
```

## Customization

### Adding Routes

1. **Create the handler** in `src/handler/mod.rs` (or a sub-module):

    ```rust
    /// List all items
    ///
    /// Returns a paginated list of items for the authenticated user.
    #[utoipa::path(
        get,
        path = "/items",
        tag = "Items",
        operation_id = "list_items",
        responses(
            (status = 200, description = "Items retrieved", body = Vec<item::Response>),
            (status = 401, description = "Unauthorized", body = schemas::error::Response),
        ),
        security(("bearer_auth" = []))
    )]
    pub async fn list_items() -> impl IntoResponse {
        todo!()
    }
    ```

2. **Register the route** in `src/main.rs`:

    ```rust
    let app = Router::new()
        .route("/health", get(handler::health))
        .route("/items", get(handler::list_items));
    ```

3. **Register with Swagger** in `src/doc.rs`:

    ```rust
    #[openapi(
        paths(
            handler::health,
            handler::list_items,   // ← add here
        ),
        components(
            schemas(
                schemas::item::Response,  // ← and here
            )
        ),
        tags(
            (name = "Items", description = "Item management"),  // ← and here
        ),
    )]
    ```

> See `src/handler/mod.rs` for commented-out examples covering GET with path/query params, POST with request body, and security annotations.

### Error Handling

Add new variants to the `AppError` enum in `src/error.rs`:

```rust
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // ... existing variants

    #[error("{user_message}")]
    CustomError {
        user_message: String,
        #[source]
        source: Option<anyhow::Error>,
    },
}
```

### Configuration

Add new environment variables by extending the `Env` struct in `src/config/env.rs`:

```rust
#[derive(Debug, Deserialize)]
pub struct Env {
    // ... existing fields
    pub database_url: String,
}
```

Then add the variable to `.env` and `.env.example`.

### API Documentation (Swagger)

This template uses **utoipa** for OpenAPI spec generation and **utoipa-swagger-ui** to serve the interactive docs.

Key conventions:

| Concern | Where | How |
|---|---|---|
| Route docs | `#[utoipa::path(...)]` on handlers | Use doc comments for summary/description |
| Grouping | `tag = "TagName"` | Match the `tags(...)` list in `doc.rs` |
| Auth | `security(("bearer_auth" = []))` | Omit for public routes |
| Schemas | `#[derive(ToSchema)]` on types | Use `#[schema(example = ...)]` for examples |
| Params | `#[derive(IntoParams)]` on query/path structs | Use `#[param(example = ...)]` for hints |

See `src/schemas/health.rs` for commented-out tips on schema documentation patterns.

## Contributing

Contributions are welcome! Please feel free to submit a pull request.
