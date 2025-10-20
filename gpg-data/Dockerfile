FROM debian:stable-slim AS builder

ARG DEBIAN_FRONTEND=noninteractive
ARG GNUPGHOME="/var/lib/greenbone/gpg"

ENV GNUPGHOME=${GNUPGHOME}

COPY *.public.key /tmp/
COPY ownertrust.txt /tmp/

RUN apt-get -y update && apt-get -y upgrade && \
    apt-get --no-install-recommends --no-install-suggests -y install \
    adduser \
    gpg \
    gpg-agent

RUN addgroup --gid 1001 --system gvm && \
    adduser --no-create-home --shell /bin/false --disabled-password --uid 1001 --system --group gvm && \
    mkdir -p ${GNUPGHOME} && \
    chown gvm:gvm ${GNUPGHOME} && \
    chmod 750 ${GNUPGHOME}

USER gvm

RUN gpg --import /tmp/community.public.key && \
    gpg --import /tmp/enterprise.public.key && \
    gpg --import-ownertrust < /tmp/ownertrust.txt

FROM busybox:stable

ARG storage_path="/var/lib/greenbone/gpg"
ARG mount_path="/mnt"

ENV STORAGE_PATH=${storage_path}
ENV MOUNT_PATH=${mount_path}

COPY --from=builder ${STORAGE_PATH} ${STORAGE_PATH}

COPY init.sh /bin/init.sh

CMD [ "/bin/init.sh" ]
