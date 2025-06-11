FROM clux/muslrust:stable-2024-11-22 AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/static AS runner

WORKDIR /app

COPY ./migrations ./migrations
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/vecko_meny_api vecko_meny_api

EXPOSE 8000/tcp
CMD ["/app/vecko_meny_api"]