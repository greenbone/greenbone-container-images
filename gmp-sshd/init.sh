#!/bin/sh
set -e

# Create the SSH host key
if ! [ -f 'host_keys/ssh_host_ed25519_key' ]; then
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

# If authorized_keys exists, restrict SSH access to only allow Unix socket forwarding.
# This is a security measure to prevent interactive logins and other forwarding types.
# Append SSH key restrictions:
#  - Only allow Unix socket forwarding
#  - Disable agent, port, pty, user rc, and X11 forwarding
#  - Only permit opening stream-local connections to /run/gvmd/*.sock
if [ -f '.ssh/authorized_keys' ]; then
    awk '{print $0 " echo '\''Only Unix socket forwarding is allowed'\''\",no-agent-forwarding,no-port-forwarding,no-pty,no-user-rc,no-X11-forwarding,permitopen=\"stream-local:/run/gvmd/*.sock\""}' ~/.ssh/authorized_keys > ~/.ssh/authorized_keys.new
    mv ~/.ssh/authorized_keys.new ~/.ssh/authorized_keys
    chmod 0600 ~/.ssh/authorized_keys
fi

# Start SSH daemon in foreground, with logging to stderr, on port 2222
/usr/sbin/sshd -D -e -f ~/sshd_config
