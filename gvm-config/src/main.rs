use std::fs::File;

use tera::{Context, Tera};

use crate::cli::{Cli, Commands};

mod cli;

fn create_context_from_template_command(cmd: &cli::NginxCommand) -> Context {
    let mut context = Context::new();
    let nginx_access_control_allow_origin_header = cmd
        .nginx_access_control_allow_origin_header
        .clone()
        .unwrap_or_else(|| format!("https://{}:{}", &cmd.nginx_host, &cmd.nginx_https_port));
    context.insert("enable_feed_key_service", &cmd.enable_feed_key_service);
    context.insert("nginx_host", &cmd.nginx_host);
    context.insert("nginx_http_port", &cmd.nginx_http_port);
    context.insert("nginx_https_port", &cmd.nginx_https_port);
    context.insert("nginx_server_certificate", &cmd.nginx_server_certificate);
    context.insert("nginx_server_key", &cmd.nginx_server_key);
    context.insert(
        "nginx_access_control_allow_origin_header",
        &nginx_access_control_allow_origin_header,
    );
    context.insert(
        "nginx_content_security_policy_header",
        &cmd.nginx_content_security_policy_header,
    );
    context.insert(
        "nginx_strict_transport_security_header",
        &cmd.nginx_strict_transport_security_header,
    );
    context.insert(
        "nginx_x_frame_options_header",
        &cmd.nginx_x_frame_options_header,
    );
    context
}

fn main() {
    let cli = Cli::default();

    match cli.command {
        Commands::NginxConfig(cmd) => {
            let context = create_context_from_template_command(&cmd);
            let destination = cmd.destination;
            let source = cmd.source;
            if !source.exists() || !source.is_dir() {
                println!(
                    "Error: The source path '{}' for the templates does not exist or is not a directory.",
                    source.display()
                );
                std::process::exit(1);
            }
            let tera = match Tera::new(&format!("{}/**/*.template", source.display())) {
                Ok(t) => t,
                Err(e) => {
                    println!("Error while parsing templates: {}", e);
                    std::process::exit(1);
                }
            };

            if !destination.exists() {
                if let Err(e) = std::fs::create_dir_all(&destination) {
                    println!(
                        "Error: Failed to create destination directory '{}': {}",
                        destination.display(),
                        e
                    );
                    std::process::exit(1);
                }
            } else if destination.exists() && !destination.is_dir() {
                println!(
                    "Error: The destination path '{}' is not a directory.",
                    destination.display()
                );
                std::process::exit(1);
            }

            let mut has_error = false;
            for name in tera.get_template_names() {
                let mut rendered_path = destination.join(name);
                rendered_path.set_extension("");
                let rendered_file = File::create(&rendered_path);
                if rendered_file.is_err() {
                    println!(
                        "Error: Failed to create file '{}': {}",
                        rendered_path.display(),
                        rendered_file.err().unwrap()
                    );
                    has_error = true;
                    continue;
                }

                match tera.render_to(name, &context, &mut rendered_file.unwrap()) {
                    Ok(_) => {
                        println!("Rendered '{}'", rendered_path.display());
                    }
                    Err(e) => {
                        println!("Error while rendering template '{}': {}", name, e);
                        has_error = true;
                    }
                }
            }

            if has_error {
                std::process::exit(1);
            }
        }
    }
}
