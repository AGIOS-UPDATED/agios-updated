# Midday Engine - Rust Backend

This is the Rust implementation of the Midday Engine backend service.

## Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Docker (optional)

## Setup

1. Install Rust using rustup:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repository and navigate to the rust-engine directory:
```bash
cd apps/rust-engine
```

3. Copy the example environment file and configure it:
```bash
cp .env.example .env
```

4. Install development dependencies:
```bash
cargo install sqlx-cli
cargo install cargo-watch
```

5. Set up the database:
```bash
sqlx database create
sqlx migrate run
```

## Development

Run the development server with auto-reload:
```bash
cargo watch -x run
```

## Build

Build for production:
```bash
cargo build --release
```

## API Documentation

Once the server is running, you can access the Swagger UI documentation at:
```
http://localhost:8080/swagger-ui/
```

## Features

- RESTful API endpoints
- JWT Authentication
- OpenAPI/Swagger documentation
- Database integration with PostgreSQL
- Middleware for logging, authentication, and error handling
- Type-safe request/response handling

## Project Structure

```
src/
├── main.rs           # Application entry point
├── config/           # Configuration management
├── handlers/         # Request handlers
├── middleware/       # Custom middleware
├── models/          # Database models
├── routes/          # Route definitions
└── utils/           # Utility functions
```

## Testing

Run tests:
```bash
cargo test
```

## Contributing

1. Create a new branch for your feature
2. Make your changes
3. Run tests and ensure they pass
4. Submit a pull request
