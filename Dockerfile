
FROM rust

WORKDIR /usr/src/wuw

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Runtime image
FROM debian:buster-slim
COPY --from=0 /usr/src/wuw/target/release/wuw /usr/local/bin/wuw
CMD ["wuw"]