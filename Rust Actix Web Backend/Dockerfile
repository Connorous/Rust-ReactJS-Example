FROM rust:1.88 as builder
WORKDIR /app
COPY . .
COPY .sqlx ./.sqlx

ENV SQLX_OFFLINE=true


RUN apt-get update && apt-get install -y pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*
RUN cargo build --release
ENV PORT=8080


FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/my-backend .
EXPOSE 8080
CMD ["./my-backend"]
