use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use std::io::{self, prelude::*};

use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Tls {
    pub host: String,

    pub enable: bool,
    
    #[serde(default)]
    pub key: PathBuf,

    #[serde(default)]
    pub cert: PathBuf,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub host: String,

    #[serde(default)]
    pub tls: Tls
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Static {
    #[serde(default)]
    pub directory: PathBuf,

    #[serde(default)]
    pub served_from: String,

    #[serde(default)]
    pub error_pages: HashMap<i16, PathBuf>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    #[serde(default)]
    pub partials: HashMap<String, PathBuf>,

    #[serde(default)]
    pub variables: HashMap<String, String>
}

// configuration toggles
const fn enabled() -> bool { true }
const fn disabled() -> bool { false }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "enabled")]
    pub enable_hot_reload: bool,

    #[serde(default = "enabled")]
    pub fast_mem_cache: bool,

    #[serde(default = "enabled")]
    pub enable_cache_control: bool,

    #[serde(default = "disabled")]
    pub enable_directory_listing: bool,

    #[serde(default = "disabled")]
    pub minify_html: bool,

    #[serde(default = "disabled")]
    pub follow_symlinks: bool,

    #[serde(default = "disabled")]
    pub enable_logging: bool
}

/// secure/fallback defaults
impl Default for Config {
    fn default() -> Self {
        Self {
            enable_hot_reload: true,
            fast_mem_cache: true,
            enable_cache_control: true,
            enable_directory_listing: false,
            minify_html: false,
            follow_symlinks: false,
            enable_logging: false
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub server: Server,
    pub routes: HashMap<String, PathBuf>,

    #[serde(default)]
    pub r#static: Static,

    #[serde(default)]
    pub template: Template,

    #[serde(default)]
    pub config: Config,

    #[serde(default)]
    pub insert_headers: HashMap<String, String>
}

use once_cell::sync::Lazy;
use parking_lot::Mutex;

/// A universal config state
pub static CONFIG_STATE: Lazy<Mutex<ServerConfig>> = Lazy::new(|| {
    Mutex::new(ServerConfig::default())
});

impl ServerConfig {
    /// Read and serialize the config file.
    pub fn read(config_file: &PathBuf) -> io::Result<Self> {
        let config_file = File::open(config_file)?;
        let buf_reader = BufReader::new(config_file);
        let config: ServerConfig = serde_json::from_reader(buf_reader)?;
        
        // update global config state
        *CONFIG_STATE.lock() = config.to_owned();

        Ok(config)
    }

    /// Generate a boilerplate binserve configuration file.
    pub fn generate_default_config(config_file: &PathBuf) -> io::Result<()> {
        if !Path::new(config_file).exists() {
            // this is better than deserializing the `default()`
            // the inlined file has readable formatting.
            let config = include_bytes!("config.json");
            let mut file = File::create(config_file)?;
            file.write_all(config)?;
        }

        Ok(())
    }
}