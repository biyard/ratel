FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    g++ \
    pkg-config \
    libfreetype6-dev \
    libfontconfig1-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY ./fetcher ./bootstrap

EXPOSE 3000

CMD ["./bootstrap"]
