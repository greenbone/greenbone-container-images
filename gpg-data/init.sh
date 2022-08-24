#!/bin/sh
set -e

if [ -d "${MOUNT_PATH}" ]; then
    rm -rf "${MOUNT_PATH}/"*
    cp -a "${STORAGE_PATH}/"* "${MOUNT_PATH}"
    chown 1001:1001 "${MOUNT_PATH}"
    echo 'files copied'
else
    echo 'nothing to do'
fi
true
