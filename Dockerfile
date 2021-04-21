FROM rust:1.51-alpine3.13 AS base

ENV USER=root

RUN apk add --no-cache musl-dev
RUN apk add --no-cache openssl-dev

FROM base as planner
WORKDIR /code
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM base as cacher
WORKDIR /code
RUN cargo install cargo-chef
COPY --from=planner /code/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM base as builder
WORKDIR /code
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /code/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release --bin cluster-node-app

FROM alpine:3.13

COPY --from=builder /code/target/release/cluster-node-app /usr/bin/cluster-node-app

EXPOSE 8080

ENTRYPOINT [ "/usr/bin/cluster-node-app" ]