FROM rust:1.83.0-alpine AS builder

WORKDIR /app
COPY . .
RUN apk add --no-cache build-base musl-dev
RUN cargo build --release

FROM scratch

COPY --from=builder /app/target/release/slack-approval /slack-approval
CMD [ "/slack-approval" ]
