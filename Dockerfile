FROM rust:1.83.0-alpine AS builder

RUN apk add --no-cache build-base musl-dev

ARG WORKDIR=/app
WORKDIR ${WORKDIR}

ENV CARGO_INCREMENTAL=0
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=${WORKDIR}/target \
    cargo build --release

RUN pwd && ls -la

FROM scratch

COPY --from=builder /app/target/release/slack-approval /slack-approval
CMD [ "/slack-approval" ]
