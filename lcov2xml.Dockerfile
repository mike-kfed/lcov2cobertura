FROM rust:1.85 AS builder

WORKDIR /usr/src/app

# Copy the project files
COPY . .

RUN cargo build --release --bin lcov2xml

# Final image
FROM cgr.dev/chainguard/glibc-dynamic AS lcov2xml-final

COPY --from=builder /usr/src/app/target/release/lcov2xml /usr/local/bin/lcov2xml

ENTRYPOINT ["/usr/local/bin/lcov2xml"]
