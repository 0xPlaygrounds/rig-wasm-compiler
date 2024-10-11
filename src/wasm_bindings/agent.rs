use wasm_bindgen::prelude::*;
use rig::agent::Agent;
use rig::providers::Provider;
use crate::adapters::http_client::WasmHttpClient;
use crate::config::ProviderConfig;

#[wasm_bindgen]
pub struct WasmAgent {
    inner: Agent<Box<dyn Provider>>,
}

#[wasm_bindgen]
impl WasmAgent {
    #[wasm_bindgen(constructor)]
    pub fn new(config_json: &str) -> Result<WasmAgent, JsValue> {
        let config: ProviderConfig = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse config: {}", e)))?;

        let http_client = WasmHttpClient::new();
        let provider = create_provider(config, http_client)
            .map_err(|e| JsValue::from_str(&format!("Failed to create provider: {}", e)))?;

        let agent = Agent::new(provider);

        Ok(WasmAgent { inner: agent })
    }

    pub async fn process(&self, input: &str) -> Result<String, JsValue> {
        self.inner.process(input)
            .await
            .map_err(|e| JsValue::from_str(&format!("Agent processing error: {}", e)))
    }

    #[wasm_bindgen(js_name = addContext)]
    pub fn add_context(&mut self, context: &str) {
        self.inner.add_context(context);
    }

    #[wasm_bindgen(js_name = clearContext)]
    pub fn clear_context(&mut self) {
        self.inner.clear_context();
    }
}

fn create_provider(config: ProviderConfig, http_client: WasmHttpClient) -> Result<Box<dyn Provider>, Box<dyn std::error::Error>> {
    // This function would create the appropriate provider based on the configuration
    // For now, we'll just create a dummy provider
    
    struct DummyProvider;
    impl Provider for DummyProvider {
        async fn process(&self, input: &str) -> Result<String, Box<dyn std::error::Error>> {
            Ok(format!("Processed: {}", input))
        }
    }

    Ok(Box::new(DummyProvider))
}