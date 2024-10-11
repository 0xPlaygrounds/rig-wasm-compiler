use wasm_bindgen_test::*;
use rig_wasm_compiler::wasm_bindings::agent::WasmAgent;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_wasm_agent() {
    let agent = WasmAgent::new("{\"api_key\": \"test_key\", \"model\": \"gpt-3.5-turbo\"}").unwrap();
    let result = agent.process("Hello, World!").await.unwrap();
    assert!(!result.is_empty(), "Agent should return a non-empty response");
}

#[wasm_bindgen_test]
async fn test_rag_example() {
    let result = rig_wasm_compiler::examples::rag_agent::run_rag_example().await;
    assert!(result.is_ok(), "RAG example should run without errors");
}