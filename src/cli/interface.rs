use std::path::PathBuf;
use clap::{Parser, Subcommand};
use tracing::Level;

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

    /// Verbosty of log files 
    #[arg(short, long, default_value = "INFO")]
    pub verbosity: Level,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Serve,
    Build,
    DryRun
}
