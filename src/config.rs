use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub wasm_pack_args: Vec<String>,
    pub output_path: Option<PathBuf>,
}

pub fn load_config(path: &PathBuf) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}