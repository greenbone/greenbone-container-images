# gvm-config Container Image

The gvm-config container image provides configuration and setup utilities for
Greenbone Vulnerability Management (GVM) environments. It currently creates
self-signed certificates and a [nginx] configuration based on environment
variables and templates.

## Usage

This image is used as an initialization container within GVM deployments to
setup the web application [GSA] correctly.

## Settings

The following settings can be adjusted for the gvm-config service.

| Environment Variable                     | Type    | Default                                 | Descriptions                                                    |
| ---------------------------------------- | ------- | --------------------------------------- | --------------------------------------------------------------- |
| TEMPLATE_DESTINATION                     | Path    | `out`                                   | A destination path for the rendered templates                   |
| TEMPLATE_SOURCE                          | Path    | `templates`                             | A source path where to look for the to be rendered templates    |
| ENABLE_FEED_KEY_SERVICE                  | Boolean | false                                   | Enable serving the feed-key-service                             |
| NGINX_ENABLE_HTTP_REDIRECT               | Boolean | true                                    | Enable redirect from http to https                              |
| NGINX_ENABLE_HTTP                        | Boolean | false                                   | Instead of redirecting to https serve the content also on https |
| NGINX_HOST                               | String  | `localhost`                             | Host or IP address to listen on                                 |
| NGINX_HTTP_PORT                          | Int     | 9392                                    | Port to listen on for http traffic                              |
| NGINX_HTTPS_PORT                         | Int     | 443                                     | Port to listen on for https traffic                             |
| NGINX_SERVER_CERTIFICATE                 | String  | `/etc/nginx/certs/server.cert.pem`      | Path to the TLS server certificate                              |
| NGINX_SERVER_KEY                         | Path    | `/etc/nginx/certs/server.key`           | Path to the TLS server key                                      |
| NGINX_ACCESS_CONTROL_ALLOW_ORIGIN_HEADER | String  | `https://$NGINX_HOST:$NGINX_HTTPS_PORT` | Value for the Access-Control-Allow-Origin header                |
| NGINX_CONTENT_SECURITY_POLICY_HEADER     | String  | [^1]                                    | Value for the Content-Security-Policy header                    |
| NGINX_STRICT_TRANSPORT_SECURITY_HEADER   | String  | `max-age=31536000; includeSubDomains;`  | Value for the Strict-Transport-Security header.                 |
| NGINX_X_FRAME_OPTIONS_HEADER             | String  | `SAMEORIGIN`                            | Value for the X-Frame-Options header                            |
| ENABLE_NGINX_CONFIG                      | Boolean | false [^2]                              | Generate the nginx configuration                                |
| ENABLE_TLS_GENERATION                    | Boolean | false [^2]                              | Generate self-signed TLS certificates                           |

[^1]: `default-src 'none'; object-src 'none'; base-uri 'none'; connect-src 'self'; script-src 'self'; script-src-elem 'self' 'unsafe-inline';frame-ancestors 'none'; form-action 'self'; style-src-elem 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; font-src 'self';img-src 'self' blob: data:;`
[^2]: Evaluated in the [init.sh](./init.sh) script

[nginx]: https://nginx.org
[GSA]: https://github.com/greenbone/gsa
