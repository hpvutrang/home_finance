# Compile stage 
FROM rust:latest as builder

WORKDIR /usr/src/finance
COPY . .
RUN cargo build --release

# Build stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates libc6  --no-install-recommends && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/finance/target/release/finance /usr/local/bin/finance
WORKDIR /usr/local/bin
EXPOSE ${HOST_PORT} 

# Run 
CMD ["/usr/local/bin/finance"]
