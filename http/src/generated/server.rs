//! Auto-generated server handlers from TypeSpec.
//! DO NOT EDIT.
//!
//! Implement the handler traits and use the generated router.

#![allow(unused_imports)]

use super::models::*;
use super::enums::*;
use async_trait::async_trait;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::routing::{delete, get, patch, post, put};
use axum::{Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;


#[derive(Debug, serde::Serialize)]
pub struct ApiError {
    pub status: u16,
    pub code: String,
    pub message: String,
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}


#[async_trait]
pub trait IngestionServiceHandler: Send + Sync + 'static {
    async fn ingest_events(&self, body: Vec<EnrichedEvent>) -> Result<BatchResponse, ApiError>;
}

async fn ingestion_service_ingest_events<S: IngestionServiceHandler>(
    State(state): State<Arc<S>>,
    Json(body): Json<Vec<EnrichedEvent>>,
) -> Result<Json<BatchResponse>, ApiError> {
    let result = state.ingest_events(body).await?;
    Ok(Json(result))
}

pub fn ingestion_service_routes<S: IngestionServiceHandler>() -> Router<Arc<S>> {
    Router::new()
        .route("/events/batch", post(ingestion_service_ingest_events::<S>))
}

pub fn create_router<S: IngestionServiceHandler>() -> Router<Arc<S>> {
    ingestion_service_routes()
}
