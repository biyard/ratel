FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app
COPY ./space-stream-worker ./bootstrap

EXPOSE 3000

CMD ["./bootstrap"]
