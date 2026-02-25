FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    g++ \
    pkg-config \
    libfreetype6-dev \
    libfontconfig1-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY . .

EXPOSE 8080

CMD ["./app-shell"]
