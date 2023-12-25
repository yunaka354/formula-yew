FROM rust:1.70 as builder

WORKDIR /usr/src/docker_rust
COPY . .

RUN cargo install --path .

# Final stage
FROM debian:bullseye-slim
COPY --from=builder /usr/local/cargo/bin/docker_rust /usr/local/bin/docker_rust

EXPOSE 3000

CMD ["docker_rust"]