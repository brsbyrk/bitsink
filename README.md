# BitSink (WIP)

> **Note:** This project is a work in progress (WIP) and is not yet ready for production use. Features and documentation are subject to change.

BitSink is a lightweight and extensible backend system designed to handle project-specific routes, authentication, and cross-origin resource sharing (CORS) policies. Each project in BitSink is uniquely identifiable, allowing flexible configurations such as custom routes, methods, and origins. BitSink is ideal for building modular APIs with per-project access control.

---

## Features

- **Dynamic Projects**:
  - Create projects with unique IDs and short IDs for simplified URL handling.
  - Add custom routes to each project.
- **Authentication**:
  - Token-based authentication using JWT (JSON Web Tokens).
  - Project-specific secret keys for secure validation.
- **CORS Configuration**:
  - Customizable allowed origins, methods, and headers per project.
  - Default support for common CORS use cases.
- **Default Routes**:
  - Each project includes `/`, `/ping`, and `/health` routes by default.
- **Extensible Middleware**:
  - Easily add custom middleware for authentication or other logic.

---

## Usage

Creating a New Project

Create a new project programmatically:

```rust
use bitsink::project::Project;

let project = Project::new(
    "My Project".to_string(),
    "This is a sample project.".to_string(),
);
println!("Project ID: {}", project.id());
println!("Short ID: {}", project.short_id());
println!("Token: {}", project.token);
```

### Default Routes

Each project includes the following default routes:
• /: Returns a friendly message with the project name.
• /ping: Health check endpoint that responds with pong.
• /health: Reports the server’s health status.

### Adding Routes

You can add custom routes to a project:

```rust
project.add_route(
    "/custom",
    axum::routing::get(|| async { "Custom route response" }),
);
```

### Authentication

Authentication is handled using Bearer tokens. Each project generates a token upon creation. Include the token in the Authorization header:

```bash
curl -X GET "http://127.0.0.1:3000/project_id/custom" \
     -H "Authorization: Bearer <project_token>"
```

### CORS Configuration

Customize CORS settings per project:

```rust
project.allow_origin("http://example.com".parse().unwrap());
project.allow_method(axum::http::Method::GET);
project.allow_header(axum::http::header::HeaderName::from_static("X-Custom-Header"));
```

Or allow all origins, methods, and headers:

```rust
project.allow_defaults();
```

## Current Situation

- **Work in Progress**: This project is still in the early stages of development. Features are being added incrementally, and existing functionality is subject to change.
- **Dependencies**: Some dependencies included in the project are not yet utilized. They have been added in preparation for future functionality.
- **Data Management**: Data storage and management are not fully implemented yet. Upcoming updates will include proper models and database integration.
- **Error Handling**: Error handling is minimal at this stage and will be improved in subsequent iterations.
- **Testing**: Unit tests and integration tests are currently limited or missing. Testing will be a key focus in future updates to ensure robustness.
- **Documentation**: The documentation reflects the current state of the project but will be expanded as the application matures.
- **Known Limitations**:
  - Routes and middleware functionality are operational but not optimized for high concurrency.
  - Authentication logic is basic and will require enhancements for production use.
  - CORS configuration works per project but lacks validation for complex setups.

### Planned Features

- Database integration for persistent project and route storage.
- Admin dashboard for project management.
- Enhanced middleware with rate limiting and request validation.
- Improved authentication with support for multiple token formats (e.g., API keys).
- Comprehensive unit and integration testing.
- Static file serving for projects requiring frontends.
