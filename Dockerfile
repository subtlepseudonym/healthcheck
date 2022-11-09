FROM rust:1.65.0 AS build
WORKDIR /workspace/
RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
WORKDIR /
COPY --from=build /workspace/target/x86_64-unknown-linux-musl/release/healthcheck /healthcheck
