FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 libcrypto3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY ./main-api ./bootstrap

EXPOSE 3000

CMD ["./bootstrap"]
