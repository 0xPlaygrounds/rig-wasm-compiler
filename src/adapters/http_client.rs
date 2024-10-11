use async_trait::async_trait;
use rig_core::http_client::HttpClient;
use gloo_net::http::Request;
use wasm_bindgen::JsValue;
use std::collections::HashMap;

pub struct WasmHttpClient;

impl WasmHttpClient {
    pub fn new() -> Self {
        WasmHttpClient
    }
}

#[async_trait(?Send)]
impl HttpClient for WasmHttpClient {
    async fn get(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = Request::get(url)
            .send()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        let text = response
            .text()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        Ok(text)
    }

    async fn post(&self, url: &str, body: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = Request::post(url)
            .body(body)
            .send()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        let text = response
            .text()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        Ok(text)
    }

    async fn post_json<T: serde::Serialize + ?Sized>(
        &self,
        url: &str,
        json: &T,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let response = Request::post(url)
            .json(json)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?
            .send()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        let text = response
            .text()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        Ok(text)
    }

    async fn post_form(
        &self,
        url: &str,
        form: &HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let response = Request::post(url)
            .form(form)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?
            .send()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        let text = response
            .text()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        
        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_get_request() {
        let client = WasmHttpClient::new();
        let result = client.get("https://httpbin.org/get").await;
        assert!(result.is_ok());
    }

    #[wasm_bindgen_test]
    async fn test_post_request() {
        let client = WasmHttpClient::new();
        let result = client.post("https://httpbin.org/post", "test data").await;
        assert!(result.is_ok());
    }

    // Add more tests as needed
}