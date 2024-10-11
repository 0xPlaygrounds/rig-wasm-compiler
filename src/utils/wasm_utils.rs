use wasm_bindgen::prelude::*;
use web_sys::console;

// Set up a panic hook for better error messages in WASM
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// Log a message to the browser console
#[wasm_bindgen]
pub fn log(s: &str) {
    console::log_1(&JsValue::from_str(s));
}

// Convert a Rust Result to a JavaScript Promise
#[wasm_bindgen]
pub async fn to_promise<T, E>(future: impl std::future::Future<Output = Result<T, E>>) -> Result<JsValue, JsValue>
where
    T: serde::Serialize,
    E: std::fmt::Display,
{
    match future.await {
        Ok(value) => Ok(serde_wasm_bindgen::to_value(&value)?),
        Err(err) => Err(JsValue::from_str(&format!("Error: {}", err))),
    }
}

// Convert a JavaScript array to a Rust Vec
#[wasm_bindgen]
pub fn js_array_to_vec(array: &js_sys::Array) -> Vec<f32> {
    array.iter().map(|v| v.as_f64().unwrap_or(0.0) as f32).collect()
}

// Convert a Rust Vec to a JavaScript array
#[wasm_bindgen]
pub fn vec_to_js_array(vec: &[f32]) -> js_sys::Array {
    vec.iter().map(|&x| JsValue::from_f64(x as f64)).collect()
}

// Measure execution time of a function
#[wasm_bindgen]
pub async fn measure_time<F, Fut, T>(f: F) -> Result<JsValue, JsValue>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, JsValue>>,
    T: serde::Serialize,
{
    let start = js_sys::Date::now();
    let result = f().await?;
    let end = js_sys::Date::now();
    let duration = end - start;

    let output = serde_json::json!({
        "result": result,
        "duration_ms": duration
    });

    Ok(serde_wasm_bindgen::to_value(&output)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_js_array_to_vec() {
        let js_array = js_sys::Array::of3(&JsValue::from_f64(1.0), &JsValue::from_f64(2.0), &JsValue::from_f64(3.0));
        let rust_vec = js_array_to_vec(&js_array);
        assert_eq!(rust_vec, vec![1.0, 2.0, 3.0]);
    }

    #[wasm_bindgen_test]
    fn test_vec_to_js_array() {
        let rust_vec = vec![1.0, 2.0, 3.0];
        let js_array = vec_to_js_array(&rust_vec);
        assert_eq!(js_array.length(), 3);
        assert_eq!(js_array.get(0).as_f64(), Some(1.0));
        assert_eq!(js_array.get(1).as_f64(), Some(2.0));
        assert_eq!(js_array.get(2).as_f64(), Some(3.0));
    }

    #[wasm_bindgen_test]
    async fn test_measure_time() {
        let result = measure_time(|| async {
            // Simulate some work
            let _ = js_sys::Promise::new(&mut |resolve, _| {
                web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 100)
                    .unwrap();
            })
            .await;
            Ok(42)
        })
        .await;

        assert!(result.is_ok());
        let output: serde_json::Value = serde_wasm_bindgen::from_value(result.unwrap()).unwrap();
        assert_eq!(output["result"], 42);
        assert!(output["duration_ms"].as_f64().unwrap() >= 100.0);
    }
}