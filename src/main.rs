use anyhow::{Context, Result};
use log::{info, error};
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

mod config;
use config::Config;

#[derive(StructOpt, Debug)]
#[structopt(name = "rig-wasm-compiler")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    project_path: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    output_path: Option<PathBuf>,

    #[structopt(long)]
    release: bool,

    #[structopt(long, parse(from_os_str))]
    config: Option<PathBuf>,
}

fn main() -> Result<()> {
    env_logger::init();
    let opt = Opt::from_args();

    let config = match &opt.config {
        Some(config_path) => config::load_config(config_path)?,
        None => Config::default(),
    };

    compile_to_wasm(&opt, &config)
}

fn compile_to_wasm(opt: &Opt, config: &Config) -> Result<()> {
    info!("Starting WebAssembly compilation...");

    // Ensure wasm-pack is installed
    if !Command::new("wasm-pack").arg("--version").output().is_ok() {
        error!("wasm-pack is not installed. Please install it first.");
        anyhow::bail!("wasm-pack not found");
    }

    // Build command
    let mut cmd = Command::new("wasm-pack");
    cmd.arg("build")
        .arg("--target").arg("web")
        .current_dir(&opt.project_path);

    // Add custom arguments from config
    for arg in &config.wasm_pack_args {
        cmd.arg(arg);
    }

    if opt.release {
        cmd.arg("--release");
    }

    let output_path = opt.output_path.as_ref().or(config.output_path.as_ref());
    if let Some(output_path) = output_path {
        cmd.arg("--out-dir").arg(output_path);
    }

    // Run wasm-pack
    let output = cmd.output().context("Failed to execute wasm-pack")?;

    if !output.status.success() {
        error!("wasm-pack build failed: {}", String::from_utf8_lossy(&output.stderr));
        anyhow::bail!("WebAssembly compilation failed");
    }

    info!("WebAssembly compilation successful!");
    Ok(())
}