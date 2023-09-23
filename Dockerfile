FROM rust:latest as builder
WORKDIR /usr/src/sorume-server
COPY . .
RUN cargo install --path ./src

FROM debian:bookworm-slim
RUN apt-get update && apt install libc6 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/sorume-server /usr/local/bin/sorume-server
CMD ["sorume-server"]

EXPOSE 8080
