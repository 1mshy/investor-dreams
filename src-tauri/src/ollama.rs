use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use tauri::State; // Import Tauri's State for accessing managed state

#[tauri::command]
pub async fn ollama_generate(state: State<'_, Ollama>, prompt: &str) -> Result<String, String> {
    let ollama = state.inner();
    let model = "llama3.1".to_string();
    let res = ollama
        .generate(GenerationRequest::new(model, prompt.to_string()))
        .await;

    match res {
        Ok(res) => Ok(res.response.to_string()),
        Err(e) => Err(format!("{}", e)),
    }
}
