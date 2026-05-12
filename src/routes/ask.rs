use axum::{response::IntoResponse, Json};
use serde::Deserialize;

use crate::services::embeddings::get_embedding;
use crate::services::llm::ask_llm;
use crate::services::semantics::search_faiss;

#[derive(Deserialize)]
pub struct AskInput {
    pub question: String,
}

pub async fn ask(Json(payload): Json<AskInput>) -> impl IntoResponse {
    let question = payload.question;

    let Ok(vector) = get_embedding(&question).await else {
        return Json("Failed to embed question").into_response();
    };

    let Ok(chunks) = search_faiss(&vector, 3).await else {
        return Json("Failed to search FAISS").into_response();
    };

    if chunks.is_empty() {
        return Json("No relevant document chunks found. Upload a document first, then ask again.")
            .into_response();
    }

    let prompt = format!(
        "Answer the question using only the provided notes. If the notes do not contain the answer, say that the uploaded documents do not contain enough information.\n---\n{}\n---\nQuestion: {}\nAnswer:",
        chunks.join("\n\n"),
        question
    );

    match ask_llm(&prompt).await {
        Ok(answer) => Json(answer).into_response(),
        Err(e) => {
            eprintln!("LLM error: {}", e);
            Json("Failed to get LLM answer").into_response()
        }
    }
}
