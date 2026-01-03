FROM rust:alpine AS builder

WORKDIR /build

# Copy dependency first (at same level as service for relative path to work)
COPY lib-analytics-core ./lib-analytics-core

# Copy ingestion service
COPY adi-analytics-ingestion ./adi-analytics-ingestion

WORKDIR /build/adi-analytics-ingestion
RUN apk add --no-cache musl-dev && cargo build --release

FROM alpine:latest

RUN apk add --no-cache ca-certificates curl

COPY --from=builder /build/adi-analytics-ingestion/target/release/adi-analytics-ingestion /usr/local/bin/

EXPOSE 8094

CMD ["adi-analytics-ingestion"]
