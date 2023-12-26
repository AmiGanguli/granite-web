use std::path::PathBuf;
use clap::{Parser, Subcommand};

/// Prints an ASCII art banner to look cool!
pub fn banner() {
    eprintln!(
        "{}", format!("{} {}\n", include_str!("banner"), env!("CARGO_PKG_VERSION"))
    )
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Interface {
    /// Configuration file.
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Chroot to this location _after_ reading the config file and TLS files.
    #[arg(short, long, value_name = "FILE")]
    pub root: Option<PathBuf>,

    /// TLS Key file.
    #[arg(short='k', long, value_name = "FILE")]
    pub tls_key: Option<PathBuf>,

    /// TLS Cert file.
    #[arg(short='t', long, value_name = "FILE")]
    pub tls_cert: Option<PathBuf>,

    /// Log file. Opened before chroot.
    #[arg(short, long, value_name = "FILE")]
    pub log_file: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Serve,
    Build,
    DryRun
}

// Command-line arguments
// pub fn args() -> ArgMatches {
    // Command::new("granite")
    //     .version(VERSION)
    //     .author("Ami Ganguli <ami@dangerousminds.ai>, Mufeed VH <mufeed@lyminal.space>")
    //     .about("A fast static web server with Automatic HTTPs, routing, templating, and security in a single binary you can setup with zero code.")
    //     .arg(Arg::new("command")
    //         .help("Command to run.")
    //         .value_name("COMMAND")
    //         .required(false)
    //         .index(1))
    //     .arg(Arg::new("config")
    //         .short('c')
    //         .help("Configuration file.")
    //         .value_name("CONFIG")
    //         .required(false)
    //         .takes_value(true))
    //         .default_vals(CONFIG_FILE)
    //     .arg(Arg::new("root")
    //         .short('r')
    //         .help("Root directory for any relative paths in the config file.")
    //         .value_name("ROOT")
    //         .required(false)
    //         .takes_value(true))
    //     .arg(Arg::new("host")
    //         .short('h')
    //         .long("host")
    //         .value_name("HOST IP/DOMAIN:PORT")
    //         .help("Host to run binserve on.")
    //         .required(false)
    //         .takes_value(true))
    //     .arg(Arg::new("tls_key")
    //         .short('k')
    //         .long("key")
    //         .value_name("TLS KEY")
    //         .help("TLS key file.")
    //         .required(false)
    //         .takes_value(true))            
    //     .arg(Arg::new("tls_cert")
    //         .short('t')
    //         .long("tls")
    //         .value_name("TLS CERT")
    //         .help("TLS cert file.")
    //         .required(false)
    //         .takes_value(true))           
        // .get_matches()
// }