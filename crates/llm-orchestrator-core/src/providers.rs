// Copyright (c) 2025 LLM DevOps
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Provider trait definitions (re-exported from llm-orchestrator-providers).

// Re-export all provider traits from the providers crate
pub use llm_orchestrator_providers::{
    CompletionRequest, CompletionResponse, LLMProvider, ProviderError,
    EmbeddingProvider, EmbeddingRequest, EmbeddingResponse, EmbeddingInput,
    VectorSearchProvider, VectorSearchRequest, VectorSearchResponse, SearchResult,
    UpsertRequest, UpsertResponse, VectorRecord,
    DeleteRequest, DeleteResponse,
};
