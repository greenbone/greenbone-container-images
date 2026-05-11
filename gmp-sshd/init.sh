#!/bin/sh
set -e

# Create the SSH host key
if [ ! -f ~/host_keys/ssh_host_ed25519_key ]; then
    ssh-keygen \
        -t ed25519 \
        -a 100 \
        -N '' \
        -f ~/host_keys/ssh_host_ed25519_key

    chmod 600 ~/host_keys/ssh_host_ed25519_key
    chmod 644 ~/host_keys/ssh_host_ed25519_key.pub
fi

# If an authorized keys string is provided via environment variable,
# # write it to the authorized_keys file for the gvm user.
if [ "$OPENVAS_SSHD_AUTHORIZED_KEYS" ]; then
    echo "$OPENVAS_SSHD_AUTHORIZED_KEYS" > ~/.ssh/authorized_keys
fi

# Start SSH daemon in foreground, with logging to stderr, on port 2222
/usr/sbin/sshd -D -e -f ~/sshd_config
