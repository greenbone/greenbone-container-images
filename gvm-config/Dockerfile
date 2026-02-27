FROM rust AS builder

WORKDIR /source
COPY Cargo.* /source/
COPY src /source/src
RUN cargo install --path .

FROM debian:stable-slim

ARG storage_path="/var/lib/gvm/gvm-config/"
ARG nginx_mount_path="/mnt/nginx"
ARG state_file="${storage_path}/copying-gvm-config-files.done"

ENV STORAGE_PATH=${storage_path}
ENV NGINX_MOUNT_PATH=${nginx_mount_path}
ENV STATE_FILE=${state_file}

ENV USER_ID="1001"
ENV GROUP_ID="1001"

RUN apt-get update && apt-get install -y --no-install-recommends \
  ca-certificates \
  openssl \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/* /usr/local/bin/
COPY certs ${STORAGE_PATH}/certs
COPY templates ${STORAGE_PATH}/templates
COPY init.sh /bin/init.sh

RUN chmod 755 /bin/init.sh

HEALTHCHECK --start-period=5s CMD test -e $STATE_FILE

CMD [ "/bin/init.sh" ]
