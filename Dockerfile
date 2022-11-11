FROM rust:1.65.0 AS build
WORKDIR /workspace/
RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN rustup toolchain install nightly --target x86_64-unknown-linux-musl
RUN rustup component add rust-src --toolchain nightly --target x86_64-unknown-linux-musl
RUN cargo +nightly build \
		 -Z build-std=std,panic_abort \
		 -Z build-std-features=panic_immediate_abort \
		 --target x86_64-unknown-linux-musl \
		 --release

FROM scratch
WORKDIR /
COPY --from=build /workspace/target/x86_64-unknown-linux-musl/release/healthcheck /healthcheck
