#!/bin/sh
#
# SPDX-FileCopyrightText: 2026 Greenbone AG
#
# SPDX-License-Identifier: AGPL-3.0-or-later

set -e

CA_CERT=${CA_CERT:-ca.cert.pem}  # Default CA cert output file if not set
CA_KEY=${CA_KEY:-ca.key}  # Default CA key output file if not set
CA_DAYS=${CA_DAYS:-3650}  # Default to 10 years if not set
CA_NAME=${CA_NAME:-ACME Test CA}  # Default CN name if not set
KEY_SIZE=${KEY_SIZE:-4096}  # Default key size if not set

openssl req \
  -x509 \
  -newkey "rsa:${KEY_SIZE}" \
  -days "${CA_DAYS}" \
  -noenc \
  -keyout "${CA_KEY}" \
  -out "${CA_CERT}" \
  -subj "/CN=${CA_NAME}" \
  -config ./openssl.cnf \
  -extensions v3_ca
