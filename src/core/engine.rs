use std::path::PathBuf;

use crate::cli::interface;
use clap::Parser;

use super::{
    files, server, templates, watcher,
    routes::RouteHandle,
    config::BinserveConfig,
};

use crate::cli::messages::{Type, push_message};

pub fn init() -> anyhow::Result<()> {
    let start_time = std::time::Instant::now();
    let mut config_file = PathBuf::from("binserve.json");

    // override with cli configurations if any
    let cli_args = interface::Interface::parse();
    if let Some(config_override) = cli_args.config {
        config_file = config_override.clone();
    }

    // generate the boilerplate starter public directory
    files::generate_starter_boilerplate(&config_file)?;

    // generate the boilerplate configuration file
    BinserveConfig::generate_default_config(&config_file)?;

    // read the configuration file
    let mut config = BinserveConfig::read(&config_file)?;

    if let Some(tls_key) = cli_args.tls_key {
        config.server.tls.key = tls_key.clone();
    }
    if let Some(tls_cert) = cli_args.tls_cert {
        config.server.tls.cert = tls_cert.clone();
    }

    // prepare template partials
    let handlebars_handle = templates::render_templates(&config)?;

    // prepare routes table
    RouteHandle::add_routes(&config.routes, &handlebars_handle)?;

    let end_time = start_time.elapsed();

    if end_time.as_millis() == 0 {
        push_message(
            Type::Info,
            &format!("Build finished in {} μs ⚡", end_time.as_micros())
        )
    } else {
        push_message(
            Type::Info,
            &format!("Build finished in {} ms ⚡", end_time.as_millis())
        )
    }

    if config.server.tls.enable {
        push_message(
            Type::Info,
            "Enabled TLS (HTTPS) 🔒"
        )
    }

    if config.config.enable_logging {
        push_message(
            Type::Info,
            "Enabled logging 📜"
        )
    }

    // start the hot reloader (file wacther)
    let hot_load_config_file = config_file.clone();
    std::thread::spawn(move || {
        watcher::hot_reload_files(&hot_load_config_file)
    });

    // and finally server take off!
    server::run_server(&config_file, config)?;

    Ok(())
}