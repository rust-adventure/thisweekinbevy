FROM ubuntu:22.04
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY target/server/release/thisweekinbevy /usr/local/bin/thisweekinbevy
COPY static /opt/static
COPY target/site/pkg /opt/
COPY Cargo.toml /opt/Cargo.toml
WORKDIR /opt
ENV RUST_LOG=info
CMD ["thisweekinbevy"]