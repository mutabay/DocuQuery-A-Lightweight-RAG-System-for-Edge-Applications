from fastapi import FastAPI
from pydantic import BaseModel
from typing import List
import faiss
import numpy as np
import json

from dotenv import load_dotenv
import os


app = FastAPI()

# Load from .env file
load_dotenv()

DIM = int(os.getenv("DIM", "768"))
INDEX_FILE = os.getenv("INDEX_FILE", "index_store/index.faiss")
META_FILE = os.getenv("META_FILE", "index_store/metadata.json")

# Try to load existing index or create new
if os.path.exists(INDEX_FILE) and os.path.getsize(INDEX_FILE) > 0:
    try:
        index = faiss.read_index(INDEX_FILE)
        with open(META_FILE, "r", encoding="utf-8") as f:
            metadata = json.load(f)
    except Exception as e:
        print(f"⚠️ Failed to load existing index, creating new one: {e}")
        index = faiss.IndexFlatL2(DIM)
        metadata = []
else:
    index = faiss.IndexFlatL2(DIM)
    metadata = []
    faiss.write_index(index, INDEX_FILE)
    with open(META_FILE, "w", encoding="utf-8") as f:
        json.dump(metadata, f)

# Schema
class AddItem(BaseModel):
    embedding: List[float]
    metadata: str


class SearchQuery(BaseModel):
    query_vector: List[float]
    k: int = 3

@app.post("/add")
def add(item: AddItem):
    vector = item.embedding
    index.add(np.array([vector], dtype="float32"))
    metadata.append(item.metadata)

    # Persist
    faiss.write_index(index, INDEX_FILE)
    with open(META_FILE, "w", encoding="utf-8") as f:
        json.dump(metadata, f)

    return {"message": "Added vector and metadata", "total": len(metadata)}

@app.post("/search")
def search(query: SearchQuery):
    if index.ntotal == 0 or not metadata:
        return { "results": [] }

    vectors = np.array([query.query_vector], dtype="float32")
    D, I = index.search(vectors, query.k)

    results = []
    for i in I[0]:
        if 0 <= i < len(metadata):
            results.append(metadata[i])

    return { "results": results }
