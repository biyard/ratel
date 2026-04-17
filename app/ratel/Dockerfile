FROM ubuntu:24.04

RUN for i in 1 2 3; do apt-get update && break || sleep 5; done \
    && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    g++ \
    pkg-config \
    libfreetype6-dev \
    libfontconfig1-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .

EXPOSE 8080

CMD ["./app-shell"]
