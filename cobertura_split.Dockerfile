FROM rust:1.85 AS builder

WORKDIR /usr/src/app

# Copy the project files
COPY . .

RUN cargo build --release --bin cobertura_split

# Final image
FROM cgr.dev/chainguard/glibc-dynamic AS lcov2xml-final

COPY --from=builder /usr/src/app/target/release/cobertura_split /usr/local/bin/cobertura_split

ENTRYPOINT ["/usr/local/bin/cobertura_split"]
