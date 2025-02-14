# Rust REST API

A modern REST API built with Rust, following clean architecture principles and best practices.

## Features

- Clean Architecture implementation
- JWT Authentication
- PostgreSQL integration with SQLx
- RESTful endpoints
- Error handling
- Input validation
- Async/await support

## Tech Stack

- Actix-web: Web framework
- SQLx: PostgreSQL async driver
- Tokio: Async runtime
- Serde: Serialization/Deserialization
- JWT: Authentication
- Validator: Input validation
- Chrono: DateTime handling
- UUID: Unique identifiers

## Project Structure

```plaintext
src/
├── application/     # Application services
├── config/         # Configuration
├── db/            # Database setup
├── domain/        # Business logic and entities
├── infrastructure/ # External services implementation
├── presentation/  # HTTP handlers and routes
├── repositories/  # Data access layer
└── utils/         # Shared utilities
```

## Getting Started

### Prerequisites

- Rust (latest stable)
- PostgreSQL
- Docker (optional)

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd rust-rest
```

2. Copy the environment file:
```bash
cp .env.example .env
```

3. Update the `.env` file with your database credentials

4. Run the migrations:
```bash
sqlx database create
sqlx migrate run
```

5. Build and run the project:
```bash
cargo run
```

## API Documentation

The API will be available at `http://localhost:8080`

### Available Endpoints

- `POST /auth/register` - Register a new user
- `POST /auth/login` - Login user
- More endpoints documentation coming soon...

## Development

To run in development mode with auto-reload:

```bash
cargo watch -x run
```

## Testing

Run the tests with:

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the LICENSE file for details
```

This README provides:
1. Project overview
2. Features list
3. Technology stack
4. Project structure
5. Setup instructions
6. Development guidelines
7. API documentation
