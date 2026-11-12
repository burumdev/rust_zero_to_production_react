# ==========================================
# 1. Frontend Builder Stage
# ==========================================
FROM node:20-alpine AS frontend-builder
WORKDIR /app
RUN corepack enable && corepack prepare pnpm@latest --activate

# Copy dependency manifests first to leverage Docker layer caching
COPY frontend/package.json frontend/pnpm-lock.yaml* ./frontend/
WORKDIR /app/frontend
RUN pnpm install

# Copy the rest of the frontend code and build
COPY frontend/ .
ENV PUBLIC_API_URL http://localhost:8000/api
RUN pnpm build

# ==========================================
# 2. Rust Cargo Chef Dependency Build Stage
# ==========================================
FROM lukemathwalker/cargo-chef:latest-rust-1.99.1 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
# This will skip the frontend pnpm build in build.rs used for
# local test runs without docker. Because we did everything necessary
# in Frontend Builder
ENV DOCKER_BUILD_SKIP_FRONTEND true
RUN cargo build --release --bin zero2prod

# ==========================================
# 3. Runtime
# ==========================================
FROM debian:trixie-slim AS runtime
WORKDIR /app
RUN apt update -y \
  && apt install -y --no-install-recommends openssl ca-certificates \
  # Clean up the debian apt artifacts
  && apt autoremove -y \
  && apt clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy from other builds the necessary folders and files for runtime
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
# We only need the "dist" build folder to run in production mode
COPY --from=frontend-builder /app/frontend/dist /app/frontend/dist

ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]

