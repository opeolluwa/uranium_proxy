ARG RUST_VERSION=1.82.0
ARG APP_NAME=bookmark_gateway

FROM rust:${RUST_VERSION} AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
RUN apt-get update && apt-get install -y protobuf-compiler musl-dev
COPY . .

RUN cargo build --release

FROM ubuntu AS final

COPY --from=build /app/target/release/bookmark_gateway ./gateway

EXPOSE 4576

CMD ["./gateway"]
