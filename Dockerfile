FROM rust:latest AS builder
WORKDIR /first_project
COPY . .
# Install protobuf-compiler
RUN apt-get update && apt-get install -y protobuf-compiler
# Build application
RUN cargo build --release --bin first-project
RUN cargo install --path database --bin migrate

# We do not need the Rust toolchain to run the binary
FROM rust:1.68.2-slim-buster AS runtime
# We need libpq for postgresql diesel
RUN apt-get update && apt-get install libpq-dev pkg-config -y
# Copy the migrate script to do database migrations and insert seed data
COPY --from=builder /first_project/target/release/first-project .
RUN chmod +x /first-project
# Copy the project binary
COPY --from=builder /first_project/database/target/release/migrate .
RUN chmod +x /migrate
# Wail till postgresql is up
ADD https://github.com/ufoscout/docker-compose-wait/releases/download/2.7.3/wait /wait
RUN chmod +x /wait
# Seed same data for golang app too
ENV GO_DB_URL="postgres://gouser:password@go-database:5432/godatabase"
# Migrate and run the project
ENTRYPOINT ["sh", "-c", "./wait && ./migrate && ./first-project"]