# Rust REST API

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

## Prerequisites

- Rust (latest stable)
- PostgreSQL
- Docker (optional)

## Running Locally

1. Clone the repository:
```bash
git clone https://github.com/adisusilayasa/rust-rest-api.git
cd rust-rest
```

2. Copy the environment file:
```bash
cp .env.example .env
```

3. Update the `.env` file with your database credentials

4. Generate SQLx prepare file:
```bash
cargo sqlx prepare
```

5. Run the migrations:
```bash
sqlx database create
sqlx migrate run
```

6. Build and run the project:
```bash
cargo run
```

## Running with Docker
```
### Using Docker directly

1. Generate SQLx prepare file first:
```bash
cargo sqlx prepare
```

2. Build the Docker image:
```bash
docker build -t rust-rest-api .
```

3. Run the container:
```bash
docker run -p 8080:8080 \
  -e DATABASE_URL="postgresql://<username>:<password>@host.docker.internal:5432/rust_rest" \
  -e JWT_SECRET="your_secret_key" \
  -e PORT="8080" \
  rust-rest-api
```

## API Endpoints

- Health Check: `GET /health`
- Authentication: `POST /api/auth/login`
- User Registration: `POST /api/auth/register`
- User Profile: `GET /api/users/profile`
- Update Profile: `PUT /api/users/profile`

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `JWT_SECRET`: Secret key for JWT tokens
- `PORT`: Server port (default: 8080)