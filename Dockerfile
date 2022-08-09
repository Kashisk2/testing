
FROM rust

ARG APP_NAME

WORKDIR /usr/src/${APP_NAME}

# Download and cache deps.
COPY Cargo.toml Cargo.lock ./
RUN mkdir ./src && touch ./src/lib.rs
RUN cargo build
RUN rm -f ./src/lib.rs 

COPY src ./src
RUN cargo build --release

# Runtime image
FROM debian:buster-slim
ARG APP_NAME=APP_NAME
COPY --from=0 /usr/src/${APP_NAME}/target/release/${APP_NAME} /usr/local/bin/${APP_NAME}
CMD ["${APP_NAME}"]