ARG NGINX_VERSION=stable-trixie
FROM nginx:$NGINX_VERSION

RUN apt-get update && apt-get install --no-install-recommends --no-install-suggests -y \
  inotify-tools \
  && rm -rf /var/lib/apt/lists/*

COPY 40-auto-reload.sh /docker-entrypoint.d/40-auto-reload.sh
RUN chmod 755 /docker-entrypoint.d/40-auto-reload.sh
