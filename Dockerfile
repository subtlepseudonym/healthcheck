FROM rust:1.65.0
WORKDIR /workspace/
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=0 /workspace/target/release/healthcheck /healthcheck
