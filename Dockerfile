FROM rust:latest as builder

WORKDIR /build
RUN rustup target add x86_64-unknown-linux-musl \
    && cargo init --bin --name sablier-proxy \
    && touch -d '1970-01-01' src/main.rs
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
# cache dependencies
RUN cargo build --release --target=x86_64-unknown-linux-musl

COPY . .
# touch main.rs to invalidate build cache
RUN touch src/main.rs \
    && cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/sablier-proxy /sablier-proxy
ENTRYPOINT [ "/sablier-proxy" ]
