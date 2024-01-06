FROM rust:latest AS builder

LABEL version="1.0.0"
LABEL description="Consumet API (Axum) Docker Image"

ARG PORT=3000
ENV PORT=${PORT}

WORKDIR /app

COPY . .

RUN \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  rustup default nightly && \
  cargo build --release && \
  cp ./target/release/consumet-api /

FROM debian:bookworm-slim AS final

RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "10001" \
  appuser

COPY --from=builder /consumet-api /usr/local/bin

RUN chown appuser /usr/local/bin/consumet-api

COPY --from=builder /app/config /opt/consumet-api/config

RUN chown -R appuser /opt/consumet-api

USER appuser

WORKDIR /opt/consumet-api

ENV RUST_LOG=INFO

EXPOSE ${PORT}

ENTRYPOINT ["consumet-api"]
