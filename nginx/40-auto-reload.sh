#!/bin/sh

set -e

AUTO_RELOAD_FILES=${AUTO_RELOAD_FILES:-/etc/nginx/}

auto_reload() {
    sleep 1 # give the container some time to start up before monitoring files
    while inotifywait --event create,modify,delete --quiet -r "${AUTO_RELOAD_FILES}"
    do
        printf "Detected change in %s, reloading Nginx...\n" "${AUTO_RELOAD_FILES}"
        if nginx -t; then
            nginx -s reload
            echo "Nginx reloaded successfully."
        else
            echo "Nginx reload failed (config test failed)."
        fi
    done
}

if [ -n "${AUTO_RELOAD_FILES}" ]; then
    auto_reload &
fi
