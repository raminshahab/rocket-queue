FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y libpq-dev &&     cargo build --release

FROM debian:stable-slim
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/game-server /usr/local/bin/game-server
ENV ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=8000
EXPOSE 8000
CMD ["rocket-queue"]