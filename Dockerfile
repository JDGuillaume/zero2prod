# Bring in cargo chef at the base layer.
FROM lukemathwalker/cargo-chef:latest-rust-1 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

# Planner Stage
FROM chef as planner
COPY . .
# Generate a lock-file for our project.
RUN cargo chef prepare --recipe-path recipe.json

# Builder Stage
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application.
RUN cargo chef cook --release --recipe-path recipe.json
# As long as our dependcencies don't change, all the layers will get cached.
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin zero2prod

# Runtime Stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]