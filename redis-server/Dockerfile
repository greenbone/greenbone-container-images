FROM redis:8.6.1

# Copy in the custom Redis configuration
COPY redis-openvas.conf /etc/redis/redis.conf

# Create gvm group and adjust permissions for Redis runtime
RUN addgroup --system --gid 1001 gvm && \
    usermod -a -G gvm redis && \
    mkdir /run/redis && chown redis:gvm /run/redis

USER redis

# Ensure socket is cleaned up before starting
CMD rm -f /run/redis/redis.sock && redis-server /etc/redis/redis.conf
