use rig::agent::Agent;
use rig::providers::openai::OpenAIProvider;
use rig::http_client::HttpClient;
use rig::embeddings::{Embedding, EmbeddingModel};
use rig::vector_store::VectorStore;
use crate::adapters::vector_store::WasmVectorStore;
use tokio_with_wasm::alias as tokio;
use wasm_bindgen::prelude::*;
use serde_json::json;

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

struct DummyEmbeddingModel;

#[async_trait::async_trait(?Send)]
impl EmbeddingModel for DummyEmbeddingModel {
    async fn embed(&self, text: &str) -> Result<Embedding, Box<dyn std::error::Error>> {
        // This is a dummy implementation. In reality, you'd call an embedding API here.
        Ok(Embedding {
            vec: vec![0.1, 0.2, 0.3] // Dummy vector
        })
    }
}

#[wasm_bindgen]
pub struct RagAgent {
    agent: Agent<OpenAIProvider<WasmHttpClient>>,
    vector_store: WasmVectorStore,
    embedding_model: DummyEmbeddingModel,
}

#[wasm_bindgen]
impl RagAgent {
    #[wasm_bindgen(constructor)]
    pub fn new(api_key: &str) -> Self {
        let http_client = WasmHttpClient;
        let provider = OpenAIProvider::new(http_client, api_key.to_string(), "gpt-3.5-turbo".to_string());
        let agent = Agent::new(provider);
        let vector_store = WasmVectorStore::new();
        let embedding_model = DummyEmbeddingModel;
        RagAgent { agent, vector_store, embedding_model }
    }

    pub async fn add_document(&mut self, id: &str, content: &str) -> Result<(), JsValue> {
        let embedding = self.embedding_model.embed(content)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        self.vector_store.add_vector(id, embedding, Some(json!({"content": content})))
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub async fn process(&self, input: &str) -> Result<String, JsValue> {
        // First, get the embedding for the input
        let query_embedding = self.embedding_model.embed(input)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Search for relevant documents
        let results = self.vector_store.search_vectors(&query_embedding, 3)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        // Construct context from retrieved documents
        let context = results.iter()
            .filter_map(|(_, id)| self.vector_store.get_vector(id).ok().flatten())
            .map(|embedding| {
                let metadata = self.vector_store.get(id)
                    .and_then(|js_value| serde_wasm_bindgen::from_value::<serde_json::Value>(js_value).ok())
                    .and_then(|json| json["content"].as_str().map(String::from))
                    .unwrap_or_default();
                metadata
            })
            .collect::<Vec<String>>()
            .join("\n\n");

        // Add context to the agent
        self.agent.add_context(&format!("Relevant information:\n{}", context));

        // Process the input with the agent
        self.agent.process(input)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

#[wasm_bindgen]
pub async fn run_rag_example() -> Result<(), JsValue> {
    let mut agent = RagAgent::new("your-api-key-here");
    
    // Add some documents to the vector store
    agent.add_document("1", "The capital of France is Paris.").await?;
    agent.add_document("2", "The Eiffel Tower is located in Paris.").await?;
    agent.add_document("3", "Paris is known as the City of Light.").await?;
    
    let response = agent.process("What can you tell me about Paris?").await?;
    
    web_sys::console::log_1(&JsValue::from_str(&format!("RAG Agent response: {}", response)));
    
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut agent = RagAgent::new("your-api-key-here");
    
    // Add some documents to the vector store
    agent.add_document("1", "The capital of France is Paris.").await?;
    agent.add_document("2", "The Eiffel Tower is located in Paris.").await?;
    agent.add_document("3", "Paris is known as the City of Light.").await?;
    
    let response = agent.process("What can you tell me about Paris?").await?;
    
    println!("RAG Agent response: {}", response);
    
    Ok(())
}