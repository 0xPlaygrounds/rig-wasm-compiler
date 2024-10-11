use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasmConfig {
    // Provider-specific configurations
    pub providers: HashMap<String, ProviderConfig>,

    // Global configuration options
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub request_timeout: Option<u64>,

    // Vector store configuration (for RAG)
    pub vector_store: Option<VectorStoreConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub api_key: String,
    pub model: String,
    pub api_base_url: Option<String>,
    pub additional_params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorStoreConfig {
    pub store_type: String,
    pub store_name: String,
    pub dimension: usize,
    pub max_elements: Option<usize>,
}

impl WasmConfig {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            max_tokens: None,
            temperature: None,
            request_timeout: None,
            vector_store: None,
        }
    }

    pub fn add_provider(&mut self, name: String, config: ProviderConfig) {
        self.providers.insert(name, config);
    }

    pub fn set_vector_store(&mut self, config: VectorStoreConfig) {
        self.vector_store = Some(config);
    }

    // Builder-style methods for optional fields
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn with_request_timeout(mut self, request_timeout: u64) -> Self {
        self.request_timeout = Some(request_timeout);
        self
    }
}