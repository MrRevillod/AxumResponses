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
/// - `code = <u16>`: HTTP status code (required)
/// - `message = "<string>"`: Custom message (optional, defaults to canonical reason)
/// - `error = <field>`: Single error field to include (optional, named fields only)
/// - `errors = <field>`: Multiple errors field to include (optional, named fields only)
///
/// **For delegation:**
/// - `transparent`: Delegate to inner type's `From<T> for Json` (for wrapping other `HttpError` types)
///
/// # Example
///
/// ```rust,ignore
/// use axum_responses::HttpError;
///
/// // Domain error
/// #[derive(Debug, thiserror::Error, HttpError)]
/// pub enum AuthError {
///     #[error("Invalid credentials")]
///     #[http(code = 401)]
///     InvalidCredentials,
///
///     #[error("Forbidden: requires {role}")]
///     #[http(code = 403, error = role)]
///     Forbidden { role: String },
/// }
///
/// // Root error composing others
/// #[derive(Debug, thiserror::Error, HttpError)]
/// pub enum AppError {
///     #[error("Auth: {0}")]
///     #[http(transparent)]  // delegates to AuthError's Json conversion
///     Auth(#[from] AuthError),
///
///     #[error("IO: {0}")]
///     #[http(code = 500)]  // fixed response, hides internal details
///     Io(#[from] std::io::Error),
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
