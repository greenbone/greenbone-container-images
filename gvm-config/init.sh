#!/bin/sh
set -e

STORAGE_PATH=${STORAGE_PATH:-"/var/lib/gvm/gvm-config"}
STATE_FILE=${STATE_FILE:-"/run/gvm-config/copying-gvm-config-files.done"}

NGINX_HOST=${NGINX_HOST:-localhost}
NGINX_PORT=${NGINX_PORT:-443}
NGINX_MOUNT_PATH=${NGINX_MOUNT_PATH:-"/mnt/nginx"}
NGINX_CONFIGS_MOUNT_PATH=${NGINX_CONFIGS_MOUNT_PATH:-"${NGINX_MOUNT_PATH}/configs"}
NGINX_CERTS_MOUNT_PATH=${NGINX_CERTS_MOUNT_PATH:-"${NGINX_MOUNT_PATH}/certs"}
NGINX_SERVER_CERT=${NGINX_SERVER_CERT:-"${NGINX_CERTS_MOUNT_PATH}/server.cert.pem"}
NGINX_SERVER_KEY=${NGINX_SERVER_KEY:-"${NGINX_CERTS_MOUNT_PATH}/server.key"}

SERVER_KEY=${SERVER_KEY:-server.key}
SERVER_CERT=${SERVER_CERT:-server.cert.pem}

rm -f "${STATE_FILE}"

if [ ! -d "${STORAGE_PATH}" ]; then
    echo "gvm-config files not found at ${STORAGE_PATH}"
    exit 1
fi

printf "\nStarting gvm-config... "

if [ -d "${NGINX_CONFIGS_MOUNT_PATH}" ] && [ "${ENABLE_NGINX_CONFIG}" = "true" ]; then
        echo "Creating nginx config files at ${NGINX_CONFIGS_MOUNT_PATH}"
        cd "${STORAGE_PATH}/templates"
        /usr/local/bin/gvm-config nginx-config --source "${STORAGE_PATH}/templates" --destination "${NGINX_CONFIGS_MOUNT_PATH}"

        if [ -n "${VERBOSE}" ]; then
            echo "nginx config files content:"
            cat "${NGINX_CONFIGS_MOUNT_PATH}/"*.conf
        fi
fi

if [ -d "${NGINX_CERTS_MOUNT_PATH}" ] && [ "${ENABLE_TLS_GENERATION}" = "true" ]; then
    if [ ! -f "${NGINX_SERVER_CERT}" ] || [ ! -f "${NGINX_SERVER_KEY}" ] ; then
        echo "generating TLS certificates"
        cd "${STORAGE_PATH}/certs"

        if [ ! -f "${CA_CERT}" ] || [ ! -f "${CA_KEY}" ] ; then
            sh ./ca-certificates.sh
        fi
        if [ ! -f "${SERVER_CERT}" ] || [ ! -f "${SERVER_KEY}" ] ; then
            sh ./server-certificates.sh
        fi
        if [ ! -f "${NGINX_SERVER_CERT}" ] || [ ! -f "${NGINX_SERVER_KEY}" ] ; then
            echo "copying generated TLS certificates to ${NGINX_CERTS_MOUNT_PATH}"
            cp -v -f "${SERVER_CERT}" "${NGINX_SERVER_CERT}"
            cp -v -f "${SERVER_KEY}" "${NGINX_SERVER_KEY}"
        fi
    fi
fi

state_dir=$(dirname ${STATE_FILE})
mkdir -p "${state_dir}"
touch "${STATE_FILE}"

echo "gvm-config done."

if [ -n "${KEEP_ALIVE}" ]; then
    sleep infinity
fi
