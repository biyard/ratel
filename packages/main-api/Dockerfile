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
COPY ./main-api ./bootstrap
COPY ./dist ./dist

EXPOSE 3000

CMD ["./bootstrap"]
