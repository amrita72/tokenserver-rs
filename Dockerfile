FROM rust:1.43.0-buster as builder
WORKDIR /app
ADD . /app
ENV PATH=$PATH:/root/.cargo/bin
RUN apt-get -q update && \
    apt-get -q install -y --no-install-recommends default-libmysqlclient-dev cmake && \
    rm -rf /var/lib/apt/lists/* && \
    cd /app && \
    mkdir -m 755 bin

RUN \
    cargo --version && \
    rustc --version && \
    cargo install --path . --locked --root /app

FROM debian:buster-slim
WORKDIR /app
RUN \
    groupadd --gid 10001 app && \
    useradd --uid 10001 --gid 10001 --home /app --create-home app && \
    apt-get -q update && \
    apt-get -q install -y --no-install-recommends default-libmysqlclient-dev libssl-dev ca-certificates libcurl4 python3-venv python3-pip && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/bin /app/bin
COPY --from=builder /app/version.json /app

USER app:app

ENTRYPOINT ["/app/bin/tokenserver"]
