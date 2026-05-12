FROM debian:stable-slim

# Install required packages, clean up apt cache, and set up SSH
# - openssh-server: SSH server
# - net-tools: network utilities (e.g., ifconfig, netstat)
# - tini: lightweight init system
# - gawk: GNU awk for text processing
# - groupadd/useradd: create non-root user for security
# - install/chown: set up user directories and permission
RUN apt-get update && \
    apt-get install -y --no-install-recommends socat openssh-server net-tools tini gawk && \
    rm -rf /var/lib/apt/lists/* && \
    groupadd -g 1001 gmp && \
    useradd -m -d /home/gmp -u 1001 -g 1001 -s /bin/bash gmp && \
    passwd -d gmp && \
    install -d -o gmp -g gmp -m 700 /home/gmp/.ssh && \
    install -d -o gmp -g gmp -m 700 /home/gmp/host_keys && \
    chmod 0444 /etc/shadow

# Copy custom SSH server config and init script
# - sshd_config: custom SSH server configuration
# - init.sh: entrypoint script for container initialization
COPY --chown=0:0 --chmod=0644 sshd_config /home/gmp/sshd_config
COPY --chown=0:0 --chmod=0755 init.sh /home/gmp/init.sh

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 CMD pgrep sshd || exit 1

USER 1001:1001
EXPOSE 2222
ENTRYPOINT ["tini", "/home/gmp/init.sh"]
