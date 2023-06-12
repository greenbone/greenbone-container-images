FROM debian:buster-slim

# This will make apt-get install without question
ARG DEBIAN_FRONTEND=noninteractive

# Install core dependencies required for using doxy-coverage
RUN apt-get update && \
    apt-get install --no-install-recommends --no-install-suggests --assume-yes \
    build-essential \
    cmake \
    doxygen \
    gcc \
    git \
    make \
    python \
    python3-venv && \
    rm -rf /var/lib/apt/lists/*
