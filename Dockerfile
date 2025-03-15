FROM rust:latest AS builder

WORKDIR /build
RUN if [ "$(uname -m)" = "aarch64" ] || [ "$(dpkg --print-architecture)" = "arm64" ]; then \
        rustup target add aarch64-unknown-linux-musl && \
        echo "aarch64-unknown-linux-musl" > /tmp/target; \
    else \
        rustup target add x86_64-unknown-linux-musl && \
        echo "x86_64-unknown-linux-musl" > /tmp/target; \
    fi && \
    cargo init --bin --name sablier-proxy && \
    touch -d '1970-01-01' src/main.rs

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

# Cache dependencies
RUN TARGET=$(cat /tmp/target) && \
    cargo build --release --target=$TARGET

COPY . .

# Touch main.rs to invalidate build cache
RUN TARGET=$(cat /tmp/target) && \
    touch src/main.rs && \
    cargo build --release --target=$TARGET && \
    mkdir -p /build/target/release && \
    cp /build/target/$TARGET/release/sablier-proxy /build/target/release/

FROM scratch
COPY --from=builder /build/target/release/sablier-proxy /sablier-proxy
ENTRYPOINT [ "/sablier-proxy" ]
