#!/bin/sh
#
# SPDX-FileCopyrightText: 2026 Greenbone AG
#
# SPDX-License-Identifier: AGPL-3.0-or-later

set -e

CA_CERT=${CA_CERT:-ca.cert.pem}  # Default CA cert file if not set
CA_KEY=${CA_KEY:-ca.key}  # Default CA key file if not set
SERVER_KEY=${SERVER_KEY:-server.key}  # Default server key output file if not set
SERVER_CERT=${SERVER_CERT:-server.cert.pem}  # Default server cert output file if not set
SERVER_CSR=${SERVER_CSR:-server.csr}  # Default server CSR output file if not set
SERVER_DAYS=${SERVER_DAYS:-825}  # Default to ~2.25 years if not set
SERVER_NAME=${SERVER_NAME:-localhost}  # Default server CN name if not set
KEY_SIZE=${KEY_SIZE:-4096}  # Default key size if not set

openssl req \
  -newkey "rsa:${KEY_SIZE}" \
  -noenc \
  -keyout "${SERVER_KEY}" \
  -out "${SERVER_CSR}" \
  -subj "/CN=${SERVER_NAME}"

openssl x509 \
  -req \
  -in "${SERVER_CSR}" \
  -CA "${CA_CERT}" \
  -CAkey "${CA_KEY}" \
  -CAcreateserial \
  -days "${SERVER_DAYS}" \
  -out "${SERVER_CERT}" \
  -extfile ./openssl.cnf \
  -extensions v3_server


rm -f "${SERVER_CSR}"
