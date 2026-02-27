use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

const DEFAULT_NGINX_HOST: &str = "localhost";
const DEFAULT_NGINX_HTTP_PORT: u16 = 9392;
const DEFAULT_NGINX_HTTPS_PORT: u16 = 443;
const DEFAULT_NGINX_SERVER_CERTIFICATE: &str = "/etc/nginx/certs/server.cert.pem";
const DEFAULT_NGINX_SERVER_KEY: &str = "/etc/nginx/certs/server.key";
const DEFAULT_NGINX_CONTENT_SECURITY_POLICY_HEADER: &str = "default-src 'none'; object-src 'none'; base-uri 'none'; connect-src 'self'; script-src 'self'; script-src-elem 'self' 'unsafe-inline';frame-ancestors 'none'; form-action 'self'; style-src-elem 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; font-src 'self';img-src 'self' blob: data:;";
const DEFAULT_NGINX_STRICT_TRANSPORT_SECURITY_HEADER: &str = "max-age=31536000; includeSubDomains;";
const DEFAULT_NGINX_X_FRAME_OPTIONS_HEADER: &str = "SAMEORIGIN";

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Render nginx templates to a specified destination directory.
    NginxConfig(NginxCommand),
}

#[derive(Args)]
pub struct NginxCommand {
    /// Destination directory for the rendered templates.
    #[arg(long, env = "TEMPLATE_DESTINATION", default_value_os_t = PathBuf::from("out"))]
    pub destination: PathBuf,

    /// Source directory for the templates. Defaults to "templates" in the current working directory.
    #[arg(long, env = "TEMPLATE_SOURCE", default_value_os_t = PathBuf::from("templates"))]
    pub source: PathBuf,

    /// Optional environment variable to enable feed key service.
    #[arg(long, env = "ENABLE_FEED_KEY_SERVICE", default_value_t = false)]
    pub enable_feed_key_service: bool,

    #[arg(long, env = "NGINX_HOST", default_value_t = String::from(DEFAULT_NGINX_HOST))]
    pub nginx_host: String,

    #[arg(long, env = "NGINX_HTTP_PORT", default_value_t = DEFAULT_NGINX_HTTP_PORT)]
    pub nginx_http_port: u16,

    #[arg(long, env = "NGINX_HTTPS_PORT", default_value_t = DEFAULT_NGINX_HTTPS_PORT)]
    pub nginx_https_port: u16,

    #[arg(long, env ="NGINX_SERVER_CERTIFICATE", default_value_t = String::from(DEFAULT_NGINX_SERVER_CERTIFICATE))]
    pub nginx_server_certificate: String,

    #[arg(long, env ="NGINX_SERVER_KEY", default_value_t = String::from(DEFAULT_NGINX_SERVER_KEY))]
    pub nginx_server_key: String,

    #[arg(long, env = "NGINX_ACCESS_CONTROL_ALLOW_ORIGIN_HEADER")]
    pub nginx_access_control_allow_origin_header: Option<String>,

    #[arg(long, env = "NGINX_CONTENT_SECURITY_POLICY_HEADER", default_value_t = String::from(DEFAULT_NGINX_CONTENT_SECURITY_POLICY_HEADER))]
    pub nginx_content_security_policy_header: String,

    #[arg(long, env = "NGINX_STRICT_TRANSPORT_SECURITY_HEADER", default_value_t = String::from(DEFAULT_NGINX_STRICT_TRANSPORT_SECURITY_HEADER))]
    pub nginx_strict_transport_security_header: String,

    #[arg(long, env = "NGINX_X_FRAME_OPTIONS_HEADER", default_value_t = String::from(DEFAULT_NGINX_X_FRAME_OPTIONS_HEADER))]
    pub nginx_x_frame_options_header: String,
}

impl Default for Cli {
    fn default() -> Cli {
        Cli::parse()
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    struct WithEnv {
        key: String,
    }

    impl WithEnv {
        fn new(key: &str, value: &str) -> Self {
            unsafe { env::set_var(key, value) };
            WithEnv {
                key: key.to_string(),
            }
        }
    }

    impl Drop for WithEnv {
        fn drop(&mut self) {
            unsafe { env::remove_var(&self.key) };
        }
    }

    fn try_parse_nginx_from(args: Vec<&str>) -> Result<Cli, clap::Error> {
        Cli::try_parse_from(vec!["test", "nginx-config"].into_iter().chain(args))
    }

    fn parse_nginx_from(args: Vec<&str>) -> NginxCommand {
        let cli = try_parse_nginx_from(args).expect("Failed to parse CLI arguments");
        match cli.command {
            Commands::NginxConfig(cmd) => cmd,
        }
    }

    #[test]
    fn test_should_use_defaults() {
        let cmd = parse_nginx_from(vec![]);
        assert_eq!(cmd.destination, PathBuf::from("out"));
        assert_eq!(cmd.source, PathBuf::from("templates"));
        assert!(!cmd.enable_feed_key_service);
        assert_eq!(cmd.nginx_host, DEFAULT_NGINX_HOST);
        assert_eq!(cmd.nginx_http_port, DEFAULT_NGINX_HTTP_PORT);
        assert_eq!(cmd.nginx_https_port, DEFAULT_NGINX_HTTPS_PORT);
        assert_eq!(
            cmd.nginx_server_certificate,
            DEFAULT_NGINX_SERVER_CERTIFICATE
        );
        assert_eq!(cmd.nginx_server_key, DEFAULT_NGINX_SERVER_KEY);
        assert_eq!(
            cmd.nginx_content_security_policy_header,
            DEFAULT_NGINX_CONTENT_SECURITY_POLICY_HEADER
        );
        assert_eq!(
            cmd.nginx_strict_transport_security_header,
            DEFAULT_NGINX_STRICT_TRANSPORT_SECURITY_HEADER
        );
        assert_eq!(
            cmd.nginx_x_frame_options_header,
            DEFAULT_NGINX_X_FRAME_OPTIONS_HEADER
        );
        assert_eq!(cmd.nginx_access_control_allow_origin_header, None);
    }

    #[test]
    fn test_should_parse_destination() {
        let cmd = parse_nginx_from(vec!["--destination", "custom_out"]);
        assert_eq!(cmd.destination, PathBuf::from("custom_out"));

        let _env = WithEnv::new("TEMPLATE_DESTINATION", "custom_out");
        let cmd = parse_nginx_from(vec![]);
        assert_eq!(cmd.destination, PathBuf::from("custom_out"));
    }

    #[test]
    fn test_should_parse_source() {
        let cmd = parse_nginx_from(vec!["--source", "custom_source"]);
        assert_eq!(cmd.source, PathBuf::from("custom_source"));

        let _env = WithEnv::new("TEMPLATE_SOURCE", "custom_source");
        let cmd = parse_nginx_from(vec![]);
        assert_eq!(cmd.source, PathBuf::from("custom_source"));
    }

    #[test]
    fn test_should_parse_enable_feed_key_service() {
        let cmd = parse_nginx_from(vec!["--enable-feed-key-service"]);
        assert!(cmd.enable_feed_key_service);

        let _env = WithEnv::new("ENABLE_FEED_KEY_SERVICE", "true");
        let cmd = parse_nginx_from(vec![]);
        assert!(cmd.enable_feed_key_service);
    }

    #[test]
    fn test_should_parse_nginx_host() {
        let cmd = parse_nginx_from(vec!["--nginx-host", "example.com"]);
        assert_eq!(cmd.nginx_host, "example.com");

        let _env = WithEnv::new("NGINX_HOST", "example.com");
        let cmd = parse_nginx_from(vec![]);
        assert_eq!(cmd.nginx_host, "example.com");
    }

    #[test]
    fn test_should_parse_nginx_http_port() {
        let cmd = parse_nginx_from(vec!["--nginx-http-port", "8080"]);
        assert_eq!(cmd.nginx_http_port, 8080);

        let _env = WithEnv::new("NGINX_HTTP_PORT", "8080");
        let cmd = parse_nginx_from(vec![]);
        assert_eq!(cmd.nginx_http_port, 8080);
    }

    #[test]
    fn test_should_parse_nginx_https_port() {
        let cmd = parse_nginx_from(vec!["--nginx-https-port", "8443"]);
        assert_eq!(cmd.nginx_https_port, 8443);

        let _env = WithEnv::new("NGINX_HTTPS_PORT", "8443");
        let cmd = parse_nginx_from(vec![]);

        assert_eq!(cmd.nginx_https_port, 8443);
    }

    #[test]
    fn test_should_parse_nginx_server_certificate() {
        let cmd = parse_nginx_from(vec!["--nginx-server-certificate", "/path/to/cert.pem"]);
        assert_eq!(cmd.nginx_server_certificate, "/path/to/cert.pem");

        let _env = WithEnv::new("NGINX_SERVER_CERTIFICATE", "/path/to/cert.pem");
        let cmd = parse_nginx_from(vec![]);
        assert_eq!(cmd.nginx_server_certificate, "/path/to/cert.pem");
    }

    #[test]
    fn test_should_parse_nginx_server_key() {
        let cmd = parse_nginx_from(vec!["--nginx-server-key", "/path/to/key.pem"]);
        assert_eq!(cmd.nginx_server_key, "/path/to/key.pem");

        let _env = WithEnv::new("NGINX_SERVER_KEY", "/path/to/key.pem");
        let cmd = parse_nginx_from(vec![]);
        assert_eq!(cmd.nginx_server_key, "/path/to/key.pem");
    }

    #[test]
    fn test_should_parse_nginx_access_control_allow_origin_header() {
        let cmd = parse_nginx_from(vec![
            "--nginx-access-control-allow-origin-header",
            "https://example.com",
        ]);
        assert_eq!(
            cmd.nginx_access_control_allow_origin_header,
            Some("https://example.com".to_string())
        );

        let _env = WithEnv::new(
            "NGINX_ACCESS_CONTROL_ALLOW_ORIGIN_HEADER",
            "https://example.com",
        );
        let cmd = parse_nginx_from(vec![]);
        assert_eq!(
            cmd.nginx_access_control_allow_origin_header,
            Some("https://example.com".to_string())
        );
    }

    #[test]
    fn test_should_parse_nginx_content_security_policy_header() {
        let cmd = parse_nginx_from(vec![
            "--nginx-content-security-policy-header",
            "default-src 'self'",
        ]);
        assert_eq!(
            cmd.nginx_content_security_policy_header,
            "default-src 'self'"
        );

        let _env = WithEnv::new("NGINX_CONTENT_SECURITY_POLICY_HEADER", "default-src 'self'");
        let cmd = parse_nginx_from(vec![]);
        assert_eq!(
            cmd.nginx_content_security_policy_header,
            "default-src 'self'"
        );
    }

    #[test]
    fn test_should_parse_nginx_strict_transport_security_header() {
        let cmd = parse_nginx_from(vec![
            "--nginx-strict-transport-security-header",
            "max-age=0",
        ]);
        assert_eq!(cmd.nginx_strict_transport_security_header, "max-age=0");

        let _env = WithEnv::new("NGINX_STRICT_TRANSPORT_SECURITY_HEADER", "max-age=0");
        let cmd = parse_nginx_from(vec![]);
        assert_eq!(cmd.nginx_strict_transport_security_header, "max-age=0");
    }

    #[test]
    fn test_should_parse_nginx_x_frame_options_header() {
        let cmd = parse_nginx_from(vec!["--nginx-x-frame-options-header", "DENY"]);
        assert_eq!(cmd.nginx_x_frame_options_header, "DENY");

        let _env = WithEnv::new("NGINX_X_FRAME_OPTIONS_HEADER", "DENY");
        let cmd = parse_nginx_from(vec![]);
        assert_eq!(cmd.nginx_x_frame_options_header, "DENY");
    }
}
