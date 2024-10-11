// Implement WASM-compatible vector store
use rig::vector_store::{VectorStore, VectorStoreError};
use rig::embeddings::Embedding;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[wasm_bindgen]
pub struct WasmVectorStore {
    vectors: HashMap<String, Vec<f32>>,
    metadata: HashMap<String, JsValue>,
}

#[derive(Serialize, Deserialize)]
struct VectorEntry {
    vector: Vec<f32>,
    metadata: JsValue,
}

#[wasm_bindgen]
impl WasmVectorStore {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        WasmVectorStore {
            vectors: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: &str, vector: Vec<f32>, metadata: JsValue) {
        self.vectors.insert(id.to_string(), vector);
        self.metadata.insert(id.to_string(), metadata);
    }

    pub fn get(&self, id: &str) -> Option<JsValue> {
        let vector = self.vectors.get(id)?;
        let metadata = self.metadata.get(id)?;
        
        let entry = VectorEntry {
            vector: vector.clone(),
            metadata: metadata.clone(),
        };
        
        serde_wasm_bindgen::to_value(&entry).ok()
    }

    pub fn delete(&mut self, id: &str) -> bool {
        self.vectors.remove(id).is_some() && self.metadata.remove(id).is_some()
    }

    pub fn search(&self, query: Vec<f32>, k: usize) -> JsValue {
        let mut distances: Vec<(String, f32)> = self.vectors.iter()
            .map(|(id, vec)| (id.clone(), cosine_similarity(&query, vec)))
            .collect();
        
        distances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        let results: Vec<JsValue> = distances.into_iter()
            .take(k)
            .filter_map(|(id, score)| {
                let metadata = self.metadata.get(&id)?;
                let result = SearchResult {
                    id,
                    score,
                    metadata: metadata.clone(),
                };
                serde_wasm_bindgen::to_value(&result).ok()
            })
            .collect();
        
        serde_wasm_bindgen::to_value(&results).unwrap_or(JsValue::NULL)
    }
}

#[derive(Serialize)]
struct SearchResult {
    id: String,
    score: f32,
    metadata: JsValue,
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot_product: f32 = a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum();
    let magnitude_a: f32 = a.iter().map(|&x| x * x).sum::<f32>().sqrt();
    let magnitude_b: f32 = b.iter().map(|&x| x * x).sum::<f32>().sqrt();
    dot_product / (magnitude_a * magnitude_b)
}

impl VectorStore for WasmVectorStore {
    fn add_vector(&mut self, id: &str, vector: Embedding, metadata: Option<serde_json::Value>) -> Result<(), VectorStoreError> {
        let js_metadata = metadata.map_or(JsValue::NULL, |m| serde_wasm_bindgen::to_value(&m).unwrap_or(JsValue::NULL));
        self.add(id, vector.vec, js_metadata);
        Ok(())
    }

    fn search_vectors(&self, query: &Embedding, k: usize) -> Result<Vec<(f32, String)>, VectorStoreError> {
        let js_results = self.search(query.vec.clone(), k);
        let results: Vec<SearchResult> = serde_wasm_bindgen::from_value(js_results)
            .map_err(|e| VectorStoreError::SearchError(e.to_string()))?;
        
        Ok(results.into_iter().map(|r| (r.score, r.id)).collect())
    }

    fn get_vector(&self, id: &str) -> Result<Option<Embedding>, VectorStoreError> {
        self.vectors.get(id)
            .map(|v| Ok(Embedding { vec: v.clone() }))
            .transpose()
    }

    fn delete_vector(&mut self, id: &str) -> Result<bool, VectorStoreError> {
        Ok(self.delete(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_add_and_get() {
        let mut store = WasmVectorStore::new();
        let vector = vec![1.0, 2.0, 3.0];
        let metadata = JsValue::from_str("test metadata");
        
        store.add("test", vector.clone(), metadata.clone());
        
        let result = store.get("test").unwrap();
        let entry: VectorEntry = serde_wasm_bindgen::from_value(result).unwrap();
        
        assert_eq!(entry.vector, vector);
        assert_eq!(entry.metadata, metadata);
    }

    #[wasm_bindgen_test]
    fn test_search() {
        let mut store = WasmVectorStore::new();
        store.add("1", vec![1.0, 0.0, 0.0], JsValue::NULL);
        store.add("2", vec![0.0, 1.0, 0.0], JsValue::NULL);
        store.add("3", vec![0.0, 0.0, 1.0], JsValue::NULL);
        
        let query = vec![1.0, 1.0, 0.0];
        let results = store.search(query, 2);
        let results: Vec<SearchResult> = serde_wasm_bindgen::from_value(results).unwrap();
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].id, "1");
        assert_eq!(results[1].id, "2");
    }
}