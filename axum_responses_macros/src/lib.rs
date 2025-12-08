mod http_error;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

/// Derive macro for HTTP error enums.
///
/// Generates implementations for:
/// - `From<Self> for JsonResponse` - Converts error to JSON response
/// - `IntoResponse` - Allows returning error directly from handlers
///
/// **Note**: Use with `thiserror::Error` for `Display`, `Error`, and `#[from]`.
///
/// # Attributes
///
/// Each variant must have `#[http(...)]` with one of:
///
/// **For direct responses:**
/// - `code | status = <u16>`: HTTP status code (required)
/// - `message = "<string>"`: Custom message (optional, defaults to canonical reason)
/// - `error = <field>`: Single error field to include (optional, named fields only)
/// - `errors = <field>`: Multiple errors field to include (optional, named fields only)
///
/// **For delegation:**
/// - `transparent`: Delegate to inner type's `From<T> for Json` (for wrapping other `HttpError` types)
///
/// **Tracing:**
/// - `#[tracing(level)]`: Adds structured logging when the error occurs (optional)
///   - `level`: One of `trace`, `debug`, `info`, `warn`, `error`
///   - Generates `tracing::*!(...)` calls with error details
///   - Compatible with `RUST_LOG` for filtering
///   - Not allowed with `transparent` variants
///
/// ### Tracing Output
/// The generated logs include:
/// - `error_type`: The variant name as string
/// - `status_code`: The HTTP status code
/// - For unnamed variants (single field): `error = ?field` (debug format)
/// - For named variants: Each field as `field_name = ?field_value`
/// - Unit variants: Only `error_type` and `status_code`
///
/// # Example
///
/// ```rust,ignore
/// use axum_responses::{thiserror::Error, HttpError};
///
/// #[derive(Debug, Error, HttpError)]
/// pub enum ApiError {
///     #[error("Not found")]
///     #[http(code = 404)]
///     #[tracing(info)]  // Log: error_type="NotFound", status_code=404
///     NotFound,
///
///     #[error("Forbidden: requires {role}")]
///     #[http(code = 403, error = role)]
///     #[tracing(warn)]  // Log: error_type="Forbidden", status_code=403, role=?role
///     Forbidden { role: String },
///
///     #[error("IO Error: {0}")]
///     #[http(code = 500)]
///     #[tracing(error)]  // Log: error_type="Io", status_code=500, error=?_inner
///     Io(#[from] std::io::Error),
///
///     #[error("Auth Error: {0}")]
///     #[http(transparent)]  // Delegates to other "HttpError" derivation
///     Auth(#[from] AuthError),
/// }
/// ```
#[proc_macro_derive(HttpError, attributes(http, tracing))]
pub fn derive_http_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match http_error::derive(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}
