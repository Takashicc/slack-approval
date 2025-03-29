FROM rust:1.85.1-alpine@sha256:4333721398de61f53ccbe53b0b855bcc4bb49e55828e8f652d7a8ac33dd0c118 AS base

LABEL org.opencontainers.image.source=https://github.com/Takashicc/slack-approval
LABEL org.opencontainers.image.description="Custom action to send approval request to Slack."

ARG WORKDIR=/app
ARG TARGETARCH
RUN apk add musl-dev ca-certificates
RUN set -eux; \
  case "$TARGETARCH" in \
    "amd64") \
      wget https://github.com/mozilla/sccache/releases/download/v0.9.0/sccache-v0.9.0-x86_64-unknown-linux-musl.tar.gz \
      && tar xzf sccache-v0.9.0-x86_64-unknown-linux-musl.tar.gz \
      && mv sccache-v0.9.0-x86_64-unknown-linux-musl/sccache /usr/local/bin \
      && chmod +x /usr/local/bin/sccache \
      ;; \
    "arm64") \
      wget https://github.com/mozilla/sccache/releases/download/v0.9.0/sccache-v0.9.0-aarch64-unknown-linux-musl.tar.gz \
      && tar xzf sccache-v0.9.0-aarch64-unknown-linux-musl.tar.gz \
      && mv sccache-v0.9.0-aarch64-unknown-linux-musl/sccache /usr/local/bin/ \
      && chmod +x /usr/local/bin/sccache \
      ;; \
    *) \
      echo "Unsupported architecture: $TARGETARCH"; \
      echo "Install using cargo"; \
      cargo install sccache --version ^0.9.0 \
      ;; \
  esac
RUN cargo install cargo-chef --version ^0.1.68
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache
ENV CARGO_INCREMENTAL=0

FROM base AS planner
WORKDIR ${WORKDIR}
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef prepare --recipe-path recipe.json

FROM base AS builder
WORKDIR ${WORKDIR}
COPY --from=planner ${WORKDIR}/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --release

FROM scratch
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/target/release/slack-approval /slack-approval
CMD [ "/slack-approval" ]
