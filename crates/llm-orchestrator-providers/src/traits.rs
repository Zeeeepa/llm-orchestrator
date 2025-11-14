// Copyright (c) 2025 LLM DevOps
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Provider trait definitions.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LLM provider trait.
#[async_trait]
pub trait LLMProvider: Send + Sync {
    /// Generate a completion.
    async fn complete(&self, request: CompletionRequest) -> Result<CompletionResponse, ProviderError>;

    /// Get provider name.
    fn name(&self) -> &str;

    /// Check if provider is healthy.
    async fn health_check(&self) -> Result<(), ProviderError> {
        Ok(())
    }
}

/// Completion request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRequest {
    /// Model name.
    pub model: String,

    /// Prompt or messages.
    pub prompt: String,

    /// System prompt (optional).
    pub system: Option<String>,

    /// Temperature (0.0 - 2.0).
    pub temperature: Option<f32>,

    /// Maximum tokens to generate.
    pub max_tokens: Option<u32>,

    /// Additional parameters.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionResponse {
    /// Generated text.
    pub text: String,

    /// Model used.
    pub model: String,

    /// Tokens used.
    pub tokens_used: Option<u32>,

    /// Additional metadata.
    #[serde(flatten)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Provider error.
#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    /// HTTP request error.
    #[error("HTTP request failed: {0}")]
    HttpError(String),

    /// Authentication error.
    #[error("Authentication failed: {0}")]
    AuthError(String),

    /// Rate limit exceeded.
    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    /// Invalid request.
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Provider-specific error.
    #[error("Provider error: {0}")]
    ProviderSpecific(String),

    /// Serialization error.
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Timeout error.
    #[error("Request timed out")]
    Timeout,

    /// Unknown error.
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<serde_json::Error> for ProviderError {
    fn from(err: serde_json::Error) -> Self {
        Self::SerializationError(err.to_string())
    }
}

/// Embedding provider trait.
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    /// Generate embeddings for text(s).
    async fn embed(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse, ProviderError>;

    /// Get provider name.
    fn name(&self) -> &str;

    /// Check if provider is healthy.
    async fn health_check(&self) -> Result<(), ProviderError> {
        Ok(())
    }
}

/// Embedding request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingRequest {
    /// Model name.
    pub model: String,

    /// Input text(s) to embed (can be single string or array).
    #[serde(flatten)]
    pub input: EmbeddingInput,

    /// Optional dimension reduction (for providers that support it).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<usize>,

    /// Additional parameters.
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// Embedding input type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmbeddingInput {
    /// Single text.
    Single { input: String },

    /// Multiple texts (batch).
    Batch { input: Vec<String> },
}

/// Embedding response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingResponse {
    /// Embedding vector(s).
    pub embeddings: Vec<Vec<f32>>,

    /// Model used.
    pub model: String,

    /// Tokens used.
    pub tokens_used: Option<u32>,

    /// Additional metadata.
    #[serde(flatten)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Vector search provider trait.
#[async_trait]
pub trait VectorSearchProvider: Send + Sync {
    /// Search for similar vectors.
    async fn search(&self, request: VectorSearchRequest) -> Result<VectorSearchResponse, ProviderError>;

    /// Upsert (insert or update) vectors.
    async fn upsert(&self, request: UpsertRequest) -> Result<UpsertResponse, ProviderError>;

    /// Delete vectors by ID.
    async fn delete(&self, request: DeleteRequest) -> Result<DeleteResponse, ProviderError>;

    /// Get provider name.
    fn name(&self) -> &str;

    /// Check if provider is healthy.
    async fn health_check(&self) -> Result<(), ProviderError> {
        Ok(())
    }
}

/// Vector search request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSearchRequest {
    /// Index/collection name.
    pub index: String,

    /// Query vector.
    pub query: Vec<f32>,

    /// Number of results to return.
    pub top_k: usize,

    /// Namespace/partition (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Metadata filter (optional, provider-specific format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,

    /// Include metadata in results.
    #[serde(default = "default_true_vs")]
    pub include_metadata: bool,

    /// Include vector embeddings in results.
    #[serde(default)]
    pub include_vectors: bool,
}

fn default_true_vs() -> bool {
    true
}

/// Vector search response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorSearchResponse {
    /// Search results.
    pub results: Vec<SearchResult>,

    /// Additional metadata.
    #[serde(flatten)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// A single search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// Result ID.
    pub id: String,

    /// Similarity score (higher is better, range depends on metric).
    pub score: f32,

    /// Result metadata (if include_metadata was true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    /// Result vector (if include_vectors was true).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector: Option<Vec<f32>>,
}

/// Upsert request for inserting/updating vectors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertRequest {
    /// Index/collection name.
    pub index: String,

    /// Vectors to upsert.
    pub vectors: Vec<VectorRecord>,

    /// Namespace/partition (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// A single vector record to upsert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorRecord {
    /// Record ID.
    pub id: String,

    /// Vector embedding.
    pub vector: Vec<f32>,

    /// Metadata (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Upsert response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertResponse {
    /// Number of vectors upserted.
    pub upserted_count: usize,

    /// Additional metadata.
    #[serde(flatten)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Delete request for removing vectors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRequest {
    /// Index/collection name.
    pub index: String,

    /// Vector IDs to delete.
    pub ids: Vec<String>,

    /// Namespace/partition (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// Delete all vectors in namespace (use with caution).
    #[serde(default)]
    pub delete_all: bool,
}

/// Delete response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteResponse {
    /// Number of vectors deleted.
    pub deleted_count: usize,

    /// Additional metadata.
    #[serde(flatten)]
    pub metadata: HashMap<String, serde_json::Value>,
}
