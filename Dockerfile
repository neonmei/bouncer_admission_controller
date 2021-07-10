# Use cargo-chef to cache project's build dependencies

FROM docker.io/rust:1.53-slim-buster as cargo-chef
WORKDIR /usr/src
RUN cargo install cargo-chef

FROM docker.io/rust:1.53-slim-buster as planner
WORKDIR /usr/src
COPY --from=cargo-chef /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM docker.io/rust:1.53-slim-buster as cacher
WORKDIR /usr/src
COPY --from=cargo-chef /usr/local/cargo/bin/cargo-chef /usr/local/cargo/bin/cargo-chef
COPY --from=planner /usr/src/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM docker.io/rust:1.53-slim-buster as builder
ARG RUST_STRIP=1
ARG PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src
COPY . .
COPY --from=cacher $CARGO_HOME $CARGO_HOME
COPY --from=cacher /usr/src/target target
RUN cargo build --release --bin bouncer && \
    if test $RUST_STRIP -eq 1; then strip target/release/bouncer; fi

FROM gcr.io/distroless/cc-debian10
COPY --from=builder --chown=nonroot /usr/src/target/release/bouncer /bouncer/controller
WORKDIR /bouncer
ENTRYPOINT [ "/bouncer/controller"]
