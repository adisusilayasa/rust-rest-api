# Rust REST API

A modern REST API built with Rust, following clean architecture principles and best practices.

## Features

- Clean Architecture with Domain-Driven Design
- JWT Authentication with middleware protection
- PostgreSQL integration with SQLx
- RESTful endpoints
- Structured error handling
- Logging middleware
- Hot reload development support

## Tech Stack

- Actix-web: Web framework
- SQLx: PostgreSQL async driver
- Tokio: Async runtime
- Serde: Serialization/Deserialization
- JWT: Authentication
- bcrypt: Password hashing
- Chrono: DateTime handling
- UUID: Unique identifiers
- env_logger: Logging

## Project Structure

```plaintext
src/
├── config/           # Configuration management
├── db/              # Database connection handling
├── domains/         # Domain-driven modules
│   ├── auth/       # Authentication domain
│   │   ├── controller.rs
│   │   ├── route.rs
│   │   └── service.rs
│   └── user/       # User domain
│       ├── controller.rs
│       ├── dto.rs
│       ├── entity.rs
│       ├── repository.rs
│       ├── route.rs
│       └── service.rs
└── utils/          # Shared utilities
    ├── auth.rs     # Authentication utilities
    ├── error.rs    # Error handling
    └── middleware/ # Middleware components
```

## Getting Started

### Prerequisites

- Rust (latest stable)
- PostgreSQL
- SQLx CLI (`cargo install sqlx-cli`)

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

3. Update the `.env` file with your configuration:
```plaintext
DATABASE_URL=postgresql://app_user:password@localhost:5432/rust_rest
JWT_SECRET=your_jwt_secret_key
PORT=8080
```

4. Setup the database:
```bash
sqlx database create
sqlx migrate run
```

5. Run the project:
```bash
cargo run
```

## API Documentation

The API is available at `http://localhost:8080/api`

### Available Endpoints

#### Authentication
- `POST /api/auth/register` - Register a new user
- `POST /api/auth/login` - Login user

#### User Management
- `GET /api/user/profile` - Get user profile (Protected)
- `PUT /api/user/profile` - Update user profile (Protected)

### Authentication

Protected endpoints require a JWT token in the Authorization header:
```
Authorization: Bearer <token>
```

## Development

Run with hot reload:
```bash
cargo watch -x run
```

## Testing

Run the tests:
```bash
cargo test
```

## License

This project is licensed under the MIT License - see the LICENSE file for details