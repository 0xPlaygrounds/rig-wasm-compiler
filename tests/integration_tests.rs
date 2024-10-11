use std::process::Command;
use std::fs;
use std::path::Path;

#[test]
fn test_compile_simple_agent() {
    let output_dir = "test_output/simple_agent";
    let result = Command::new("cargo")
        .args(&["run", "--", 
                "--input", "examples/simple_agent.rs", 
                "--output", output_dir, 
                "--opt-level", "2"])
        .output()
        .expect("Failed to execute rig-wasm-compiler");

    assert!(result.status.success(), "Compilation failed: {:?}", String::from_utf8_lossy(&result.stderr));
    assert!(Path::new(&format!("{}/rig_wasm_bg.wasm", output_dir)).exists());
    assert!(Path::new(&format!("{}/rig_wasm.js", output_dir)).exists());

    // Clean up
    fs::remove_dir_all(output_dir).unwrap();
}

#[test]
fn test_compile_rag_agent() {
    let output_dir = "test_output/rag_agent";
    let result = Command::new("cargo")
        .args(&["run", "--", 
                "--input", "examples/rag_agent.rs", 
                "--output", output_dir, 
                "--opt-level", "2", 
                "--typescript"])
        .output()
        .expect("Failed to execute rig-wasm-compiler");

    assert!(result.status.success(), "Compilation failed: {:?}", String::from_utf8_lossy(&result.stderr));
    assert!(Path::new(&format!("{}/rig_wasm_bg.wasm", output_dir)).exists());
    assert!(Path::new(&format!("{}/rig_wasm.js", output_dir)).exists());
    assert!(Path::new(&format!("{}/rig_wasm.d.ts", output_dir)).exists());

    // Clean up
    fs::remove_dir_all(output_dir).unwrap();
}