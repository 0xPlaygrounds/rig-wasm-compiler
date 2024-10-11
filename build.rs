use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");

    // Ensure wasm-pack is installed
    if !Command::new("wasm-pack").arg("--version").status().unwrap().success() {
        panic!("wasm-pack is not installed. Please install it using `cargo install wasm-pack`");
    }

    // Set up wasm32-unknown-unknown target
    if !Command::new("rustup").args(&["target", "add", "wasm32-unknown-unknown"]).status().unwrap().success() {
        panic!("Failed to add wasm32-unknown-unknown target");
    }
}