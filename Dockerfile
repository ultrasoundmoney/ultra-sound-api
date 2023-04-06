FROM lukemathwalker/cargo-chef AS chef
WORKDIR /app
ARG database_url

FROM chef AS planner
COPY . .
# Figure out if dependencies have changed.
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this layer is cached for massive speed up.
RUN cargo chef cook --release --recipe-path recipe.json
# Build application - this should be re-done every time we update our src.
COPY . .

ENV DATABASE_URL $database_url
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/ultra-sound-api /usr/local/bin

# sqlx depends on native TLS, which is missing in buster-slim.
RUN apt update && apt install -y libssl1.1 ca-certificates

EXPOSE 3000
ENTRYPOINT ["/usr/local/bin/ultra-sound-api"]
