FROM ubuntu:22.04
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY target/server/release/thisweekinbevy /usr/local/bin/thisweekinbevy
COPY target/site /opt/
# COPY Cargo.toml /opt/Cargo.toml
WORKDIR /opt
ENV RUST_LOG=info
ENV LEPTOS_OUTPUT_NAME="this-week-in-bevy"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="[::]:3000"
CMD ["thisweekinbevy"]