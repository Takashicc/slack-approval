FROM rust:1.83.0-alpine AS builder

RUN apk add --no-cache build-base musl-dev

ARG WORKDIR=/app
WORKDIR ${WORKDIR}

ENV CARGO_INCREMENTAL=0
COPY . .
RUN pwd && ls -la ./
RUN cargo build --release

RUN pwd && ls -la ./

FROM scratch

COPY --from=builder /app/target/release/slack-approval /slack-approval
CMD [ "/slack-approval" ]
