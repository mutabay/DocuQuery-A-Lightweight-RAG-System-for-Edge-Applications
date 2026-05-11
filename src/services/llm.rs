use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::env;

#[derive(Serialize)]
struct LlmRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize, Debug)]
struct LlmChunk {
    response: String,
    done: bool,
}

pub async fn ask_llm(prompt: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let model = env::var("OLLAMA_LLM_MODEL").unwrap_or_else(|_| "llama3.1:8b".to_string());
    let url = env::var("OLLAMA_LLM_URL").unwrap_or_else(|_| "http://localhost:11434/api/generate".to_string());

    let req_body = LlmRequest {
        model: &model,
        prompt,
    };

    let res = client.post(&url).json(&req_body).send().await?;

    let mut answer = String::new();
    let mut stream = res.bytes_stream();

    use futures_util::StreamExt;
    while let Some(chunk) = stream.next().await {
        let data = chunk?;
        for line in String::from_utf8_lossy(&data).lines() {
            if let Ok(parsed) = serde_json::from_str::<LlmChunk>(line) {
                answer.push_str(&parsed.response);
                if parsed.done {
                    break;
                }
            }
        }
    }

    Ok(answer.trim().to_string())
}
