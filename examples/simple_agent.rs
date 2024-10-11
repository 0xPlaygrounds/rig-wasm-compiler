use rig::agent::Agent;
use rig::providers::openai::OpenAIProvider;
use rig::http_client::HttpClient;
use tokio_with_wasm::alias as tokio;
use wasm_bindgen::prelude::*;

struct WasmHttpClient;

#[async_trait::async_trait(?Send)]
impl HttpClient for WasmHttpClient {
    async fn get(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        // In a real implementation, this would use a WASM-compatible HTTP client
        Ok(format!("GET request to {}", url))
    }

    async fn post(&self, url: &str, body: &str) -> Result<String, Box<dyn std::error::Error>> {
        // In a real implementation, this would use a WASM-compatible HTTP client
        Ok(format!("POST request to {} with body: {}", url, body))
    }
}

#[wasm_bindgen]
pub struct SimpleAgent {
    agent: Agent<OpenAIProvider<WasmHttpClient>>,
}

#[wasm_bindgen]
impl SimpleAgent {
    #[wasm_bindgen(constructor)]
    pub fn new(api_key: &str) -> Self {
        let http_client = WasmHttpClient;
        let provider = OpenAIProvider::new(http_client, api_key.to_string(), "gpt-3.5-turbo".to_string());
        let agent = Agent::new(provider);
        SimpleAgent { agent }
    }

    pub async fn process(&self, input: &str) -> Result<String, JsValue> {
        self.agent.process(input)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn add_context(&mut self, context: &str) {
        self.agent.add_context(context);
    }
}

#[wasm_bindgen]
pub async fn run_example() -> Result<(), JsValue> {
    let mut agent = SimpleAgent::new("your-api-key-here");
    
    agent.add_context("You are a helpful assistant.");
    
    let response = agent.process("Hello, who are you?").await?;
    
    web_sys::console::log_1(&JsValue::from_str(&format!("Agent response: {}", response)));
    
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut agent = SimpleAgent::new("your-api-key-here");
    
    agent.add_context("You are a helpful assistant.");
    
    let response = agent.process("Hello, who are you?").await?;
    
    println!("Agent response: {}", response);
    
    Ok(())
}