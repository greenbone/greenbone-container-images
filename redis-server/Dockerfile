FROM debian:stable-slim

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get -y update && apt-get -y upgrade && \
    apt-get --no-install-recommends --no-install-suggests -y install redis

COPY redis-openvas.conf /etc/redis/redis.conf

RUN addgroup --system --gid 1001 gvm && \
    usermod -a -G gvm redis && \
    mkdir /run/redis && chown redis:gvm /run/redis

USER redis

CMD rm -f /run/redis/redis.sock && redis-server /etc/redis/redis.conf
