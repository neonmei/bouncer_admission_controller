FROM docker.io/rust:1.52.1-alpine3.13 as build_step

ARG PACKAGE_DEPS="build-base=~0.5 libffi-dev~=3.3 openssl-dev=~1.1 gcc=~10.2"
WORKDIR /bouncer

COPY . .
RUN apk add --no-cache ${PACKAGE_DEPS} && \
    cargo build --release

FROM alpine:3.13

RUN apk add --no-cache ca-certificates tzdata && \
    adduser -u 10001 -h /bouncer -D controller
COPY --chown=controller --from=build_step /bouncer/target/release/bouncer /bouncer/controller
WORKDIR /bouncer

ENTRYPOINT [ "/bouncer/controller"]
