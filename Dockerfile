FROM rust:1.85-alpine AS builder

WORKDIR /app

# Copy dependency first
COPY lib-analytics-core /lib-analytics-core

# Copy ingestion service
COPY adi-analytics-ingestion .

RUN apk add --no-cache musl-dev && cargo build --release

FROM alpine:latest

RUN apk add --no-cache ca-certificates curl

COPY --from=builder /app/target/release/adi-analytics-ingestion /usr/local/bin/

EXPOSE 8094

CMD ["adi-analytics-ingestion"]
