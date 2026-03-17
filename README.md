# 📄 DocuQuery

> Ask your documents – securely, simply, locally.

DocuQuery is a lightweight, private Retrieval-Augmented Generation (RAG) tool. It lets you ask questions over your own files (PDF, TXT, Markdown, etc.) and returns meaningful answers using local vector search and LLMs. Built with Rust for performance, and designed to run on your own machine or edge device.

---

## 🎯 Why DocuQuery?

Long documents often hide critical information. Manuals, research papers, or changelogs are hard to parse. DocuQuery solves this by enabling you to:

- Upload documents and generate vector embeddings
- Ask natural language questions
- Receive answers with cited source passages — all **securely and locally**

---

## 🧠 Why Local / Edge?

- No internet? Still usable.
- Sensitive docs? No leaks.
- Don’t want OpenAI costs? Use a local model.
- Run it on laptops, desktops, or Raspberry Pi.

---

## ✅ Functional Requirements

| ID  | Requirement                                                               |
| --- | ------------------------------------------------------------------------- |
| FR1 | User can upload documents (TXT, PDF, MD).                                 |
| FR2 | System extracts and embeds document content.                              |
| FR3 | User can send a question through a local HTTP API.                        |
| FR4 | System retrieves relevant content using vector similarity search (FAISS). |
| FR5 | System generates an answer using an LLM based on retrieved chunks.        |
| FR6 | System returns the answer plus source passages.                           |
| FR7 | System supports running via Docker.                                       |

---

## 🚫 Non-Functional Requirements

| ID   | Requirement                                                  |
| ---- | ------------------------------------------------------------ |
| NFR1 | Queries should return in under 3 s for 10-20 documents.      |
| NFR2 | No internet dependency if using local LLM and embeddings.    |
| NFR3 | Modules should be loosely coupled and easy to extend.        |
| NFR4 | Codebase should be clean, logged, and production-deployable. |


---

## 🏗️ Architecture Overview

![DocuQuery Architecture](diagrams/flowchart.png)

---

## 🔧 Technologies

| Component       | Stack                       |
| --------------- | --------------------------- |
| Backend         | Rust + Axum                 |
| Embeddings      | Ollama + `nomic-embed-text` |
| LLM             | Ollama + `llama3.2`         |
| Vector Search   | Python + FAISS + FastAPI    |
| Communication   | HTTP JSON                   |
| Document Format | `.txt` (future: PDF)        |

---

## 📂 File Structure

```text
docuquery/
├── src/
│   ├── main.rs
│   ├── routes/
│   │   ├── upload.rs
│   │   ├── query.rs
│   │   └── ask.rs
│   ├── services/
│   │   ├── embeddings.rs
│   │   ├── semantics.rs
│   │   └── llm.rs
│   └── utils/
│       ├── file.rs
│       └── chunk.rs
├── faiss_server/ (Python FastAPI)
│   ├── main.py
│   └── index_store/
├── .env
├── Dockerfile (optional later)
```

---


🔜 Next Improvements
| Feature                    | Description                                                      |
| -------------------------- | ---------------------------------------------------------------- |
| 🧠 Local LLM prompt tuning | Add roles/persona, or ask style (bullet vs paragraph)            |
| 🌍 Web UI                  | Add frontend with Tauri or React                                 |
| 💬 Streaming output        | Stream LLM answers in real-time via Axum                         |
| 💾 Save answers + history  | Store past questions and answers in a local DB                   |

---

## 🧠 Learning Goals

| Topic         | Technology/Concept           |
| ------------- | ---------------------------- |
| Rust Basics   | Ownership, async, traits     |
| Web API       | Axum, REST patterns          |
| Generative AI | RAG, LLMs, LangChain         |
| Vector Search | FAISS                        |
| Deployment    | Docker, CI/CD, Observability |


