FROM debian:stable-slim

# Install required packages, clean up apt cache, and set up SSH
# - openssh-server: SSH server
# - net-tools: network utilities (e.g., ifconfig, netstat)
# - tini: lightweight init system
# - gawk: GNU awk for text processing
# - ssh-keygen -A: generate host keys
# - groupadd/useradd: create non-root user for security
# - install/chown: set up user directories and permission
RUN apt-get update && \
    apt-get install -y --no-install-recommends openssh-server net-tools tini gawk && \
    rm -rf /var/lib/apt/lists/* && \
    ssh-keygen -A && \
    groupadd -g 10041 gvm && \
    useradd -m -d /home/gvm -s /bin/false -u 10041 -g 10041 gvm && \
    install -d -o gvm -g gvm -m 700 /home/gvm/.ssh && \
    chown gvm:gvm -R /etc/ssh

# Copy custom SSH server config and init script
# - sshd_config: custom SSH server configuration
# - init.sh: entrypoint script for container initialization
#COPY --chown=0:0 --chmod=0644 sshd_config /etc/ssh/sshd_config
#COPY --chown=0:0 --chmod=0755 init.sh /usr/bin/init.sh
COPY sshd_config /etc/ssh/sshd_config
COPY init.sh /usr/bin/init.sh
RUN chmod 0600 /etc/ssh/sshd_config && chmod 0755 /usr/bin/init.sh && chown gvm:gvm /etc/ssh/sshd_config

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 CMD pgrep sshd || exit 1

USER 10041:10041
EXPOSE 2222
ENTRYPOINT ["tini", "/usr/bin/init.sh"]
