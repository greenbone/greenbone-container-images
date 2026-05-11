# GVM SSH Forwarding Docker Container

This repository provides a Docker container that runs an OpenSSH server configured for **secure Unix socket forwarding** for the GVM (Greenbone Vulnerability Manager) user.

It is designed to allow SSH tunneling to GVM sockets while **disabling interactive login** and other forwarding types for security.

---

## Features

* Lightweight Debian-based image.
* SSH server restricted to **Unix socket forwarding**.
* Non-root GVM user for secure access.
* Preconfigured to use environment variables for SSH authorized keys.
* Health checks to ensure the SSH daemon is running.
* Examples provided for secure socket forwarding.

## Files in this Repository

* **Dockerfile** – Builds the container with OpenSSH server and non-root GVM user.
* **init.sh** – Container entrypoint that sets up `authorized_keys` and enforces SSH restrictions.
* **sshd_config** – SSH daemon configuration file to enforce security restrictions.
* **examples** – Demonstrates how to connect and forward sockets securely.

---

## Environment Variables

| Variable                       | Description                                              |
| ------------------------------ | -------------------------------------------------------- |
| `OPENVAS_SSHD_AUTHORIZED_KEYS` | Public SSH key(s) for the `gvm` user (one line per key). |

---

## Security Features

* GVM user cannot log in interactively.
* Only allows stream-local forwarding to `/run/gvmd/*.sock`.
* SSH agent forwarding, X11 forwarding, TTY, and TCP port forwarding are disabled.
* Container runs as non-root user `gvm` with UID 10041.

---

## Healthcheck

The container includes a healthcheck that verifies the SSH daemon is running:

```text
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 CMD pgrep sshd || exit 1
```

---

## Getting Started

**If you are sharing a Unix socket across multiple containers, use the Compose feature to make the group for the Unix socket accessible to the SSH server.**

```bash
services:
  sshd:
    group_add:
      - 1001 # GVMD Unix Socket Group 
```

### Build the Docker Image

```bash
docker build -t gvm-ssh-forward .
```

### Run the Container

```bash
docker run -d \
  -p 2222:2222 \
  -e OPENVAS_SSHD_AUTHORIZED_KEYS="your-ssh-public-key" \
  --name gvm-ssh gvm-ssh-forward
```

The container runs an SSH daemon on port `2222` and restricts all connections to **Unix socket forwarding only**.

---

## SSH Usage Examples

These examples assume your container is running on `172.17.0.2`.

### Forward GVM socket to local Unix socket

```bash
ssh -N -p 2222 -L /tmp/c.sock:/run/gvmd/c.sock gvm@172.17.0.2
```

* `-N` prevents running remote commands.
* `-L /tmp/c.sock:/run/gvmd/c.sock` forwards the remote GVM socket to a local socket file.

### Forward GVM socket to a local TCP port

```bash
ssh -N -p 2222 -L localhost:1337:/run/gvmd/c.sock gvm@172.17.0.2
```

* This makes the GVM socket accessible locally via TCP port `1337`.

### Notes

* Only **Unix socket forwarding** is allowed; all other SSH features are disabled.
* Interactive shell access and root login are disabled for security.

---
