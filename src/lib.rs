use wasm_bindgen::prelude::*;
use crate::config::WasmConfig;
use std::path::Path;
use std::fs;

mod config;
mod wasm_bindings;
mod adapters;
mod utils;

#[wasm_bindgen]
pub fn initialize_rig_wasm(config_json: &str) -> Result<(), JsValue> {
    // Set up the panic hook for better error messages
    utils::wasm_utils::set_panic_hook();

    // Parse the config JSON
    let config: WasmConfig = serde_json::from_str(config_json)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse config: {}", e)))?;

    // Initialize Rig with WASM-specific configuration
    // This is where we would set up providers, vector stores, etc.
    // For now, we'll just log the config
    web_sys::console::log_1(&JsValue::from_str(&format!("Initialized Rig WASM with config: {:?}", config)));

    Ok(())
}

pub fn compile_to_wasm(
    input_file: &str,
    output_dir: &str,
    opt_level: u8,
    generate_typescript: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if input file exists
    let input_path = Path::new(input_file);
    if !input_path.exists() {
        return Err(format!("Input file does not exist: {}", input_file).into());
    }

    // Create output directory if it doesn't exist
    let output_path = Path::new(output_dir);
    fs::create_dir_all(output_path)?;

    // Run wasm-pack to compile the input file
    let mut command = std::process::Command::new("wasm-pack");
    command
        .arg("build")
        .arg("--target").arg("web")
        .arg("--out-dir").arg(output_dir)
        .arg("--out-name").arg("rig_wasm")
        .arg("--").arg(input_file);

    // Set optimization level
    command.arg("-O").arg(opt_level.to_string());

    if generate_typescript {
        command.arg("--typescript");
    }

    let output = command.output()?;

    if !output.status.success() {
        return Err(format!(
            "wasm-pack failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ).into());
    }

    println!("WASM compilation successful");
    println!("Output directory: {}", output_dir);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_rig_wasm() {
        let config_json = r#"
        {
            "providers": {
                "openai": {
                    "api_key": "test_key",
                    "model": "gpt-3.5-turbo"
                }
            },
            "max_tokens": 100,
            "temperature": 0.7
        }
        "#;

        assert!(initialize_rig_wasm(config_json).is_ok());
    }

    // Add more tests as needed
}