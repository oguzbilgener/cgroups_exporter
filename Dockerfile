FROM rust:1.98-alpine AS base
ENV CARGO_NET_GIT_FETCH_WITH_CLI=true
RUN apk update && \
    apk add --no-cache openssh git build-base musl-dev openssl perl && \
    mkdir -p ~/.ssh && \
    ssh-keyscan -t rsa github.com >> ~/.ssh/known_hosts
WORKDIR /usr/src/app

FROM base AS chef
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /usr/src/app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

FROM alpine AS runtime
RUN addgroup -S cgroups_exporter && adduser -S cgroups_exporter -G cgroups_exporter
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/cgroups_exporter /usr/local/bin/
USER cgroups_exporter
CMD ["/usr/local/bin/cgroups_exporter"]