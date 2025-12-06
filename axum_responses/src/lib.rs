//! # axum_responses
//!
//! Ergonomic response builders for Axum web applications.
//!
//! This crate provides three main response types that implement `IntoResponse`:
//!
//! - [`JsonResponse`] - Standardized JSON API responses
//! - [`File`] - File downloads and inline content
//! - [`Redirect`] - HTTP redirects
//!
//! ## Quick Start
//!
//! ```rust
//! use axum_responses::JsonResponse;
//! use serde_json::json;
//!
//! // Simple OK response
//! async fn health() -> JsonResponse {
//!     JsonResponse::Ok().message("Healthy")
//! }
//!
//! // Response with data
//! async fn get_user() -> JsonResponse {
//!     JsonResponse::Ok().data(json!({"id": 1, "name": "Alice"}))
//! }
//!
//! // Error response
//! async fn not_found() -> JsonResponse {
//!     JsonResponse::NotFound().message("User not found")
//! }
//! ```
//!
//! ## Final JSON Format
//!
//! All JSON responses follow this standardized structure:
//!
//! ```json
//! {
//!     "code": 200,
//!     "success": true,
//!     "message": "Healthy",
//!     "timestamp": "2023-10-01T12:00:00Z"
//!     "data": { ... },        // Optional (present using .data())
//!     "error": { ... }       // Optional (present using .error())
//!     "errors": [ ... ]     // Optional (present using .errors())
//! }
//! ```

mod macros;
mod response;

pub mod thiserror {
    pub use thiserror::Error;
}

pub use axum_responses_macros::HttpError;
pub use response::{
    ContentDisposition, File, JsonResponse, JsonResponseBody, Redirect,
};
