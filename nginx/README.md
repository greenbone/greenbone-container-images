# nginx Container Image

A nginx container image based on `nginx:stable` to support auto reloading of
config file changes.

Because the [gvm-config](../gvm-config/README.md) container image generates a
nginx configuration on each startup, nginx needs to be restarted to load the
new and possibly changed configuration file.
