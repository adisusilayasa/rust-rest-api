FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

# Build-time arguments with default values
ARG DATABASE_URL
ARG JWT_SECRET
ARG PORT="8080"

# Set environment variables for the build
ENV DATABASE_URL=${DATABASE_URL} \
    JWT_SECRET=${JWT_SECRET} \
    PORT=${PORT} \
    SQLX_OFFLINE=true

# Build the application
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/rust_rest .

# Copy environment variables to runtime
ENV DATABASE_URL=${DATABASE_URL} \
    JWT_SECRET=${JWT_SECRET} \
    PORT=${PORT}

RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

CMD ["./rust_rest"]