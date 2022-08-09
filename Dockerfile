
FROM rust

ARG APP_NAME

WORKDIR /usr/src/${APP_NAME}

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Runtime image
FROM debian:buster-slim
ARG APP_NAME=APP_NAME
COPY --from=0 /usr/src/${APP_NAME}/target/release/${APP_NAME} /usr/local/bin/${APP_NAME}
CMD ["${APP_NAME}"]