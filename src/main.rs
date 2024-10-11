use clap::Parser;
use rig_wasm_compiler::compile_to_wasm;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input Rust file to compile
    #[clap(short, long, parse(from_os_str))]
    input: PathBuf,

    /// Output directory for WASM files
    #[clap(short, long, parse(from_os_str))]
    output: PathBuf,

    /// Optimization level (0-3)
    #[clap(short, long, default_value = "2")]
    opt_level: u8,

    /// Generate TypeScript definitions
    #[clap(short, long)]
    typescript: bool,
}

fn main() {
    let args = Args::parse();

    println!("Compiling {} to WASM...", args.input.display());

    match compile_to_wasm(
        args.input.to_str().unwrap(),
        args.output.to_str().unwrap(),
        args.opt_level,
        args.typescript,
    ) {
        Ok(()) => println!("Successfully compiled to WASM"),
        Err(e) => eprintln!("Compilation failed: {}", e),
    }
}