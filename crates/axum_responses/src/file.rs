use axum::{
    body::Body,
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse as AxumIntoResponse, Response as AxumResponse},
};

use crate::JsonResponse;

/// A specialized `Result` type for file responses.
///
/// This type alias can be used for handlers that return file responses
/// as `Ok` responses and Json Standardized error responses.
pub type FileResult = std::result::Result<FileResponse, JsonResponse>;

/// Represents a file response that can be sent to the client.
/// This can be used to serve files for download or display in the browser.
///
/// ```rust
///
/// use axum_responses::{FileResponse, ContentDisposition, FileResult};
///
/// async fn file_handler() -> FileResult {
///     let file_bytes = std::fs::read("path/to/file.txt").unwrap();
///
///     let response = FileResponse::builder()
///         .bytes(&file_bytes)
///         .content_type("text/plain")
///         .filename("file.txt")
///         .disposition(ContentDisposition::Attachment);
///
///     Ok(response)
/// }
#[derive(Debug)]
pub struct FileResponse {
    bytes: Vec<u8>,
    content_type: &'static str,
    filename: Option<&'static str>,
    disposition: ContentDisposition,
    headers: HeaderMap,
}

/// Specifies how the content should be presented to the user.
///
/// - **Inline**: The content is displayed directly in the browser.
/// - **Attachment**: The content is treated as a downloadable file.
#[derive(Debug, Default)]
pub enum ContentDisposition {
    /// Indicates that the content should be displayed inline in the browser.
    Inline,

    /// Indicates that the content should be treated as an
    /// attachment to be downloaded.
    #[default]
    Attachment,
}

impl FileResponse {
    /// Creates a new `FileResponse` builder.
    pub fn builder() -> Self {
        Self {
            bytes: Vec::new(),
            content_type: "",
            filename: None,
            disposition: ContentDisposition::Attachment,
            headers: HeaderMap::new(),
        }
    }

    /// Sets the file content as a byte slice.
    pub fn bytes(mut self, bytes: &[u8]) -> Self {
        self.bytes = bytes.to_vec();
        self
    }

    /// Sets the content type of the file.
    pub fn content_type(mut self, content_type: &'static str) -> Self {
        self.content_type = content_type;
        self
    }

    /// Adds a custom header to the file response.
    pub fn add_header(mut self, key: &'static str, value: &'static str) -> Self {
        if let (Ok(header_name), Ok(header_value)) =
            (HeaderName::try_from(key), HeaderValue::try_from(value))
        {
            self.headers.insert(header_name, header_value);
        }

        self
    }

    /// Sets the filename for the file response.
    pub fn filename(mut self, filename: &'static str) -> Self {
        self.filename = Some(filename);
        self
    }

    /// Sets the content disposition for the file response.
    pub fn disposition(mut self, disposition: ContentDisposition) -> Self {
        self.disposition = disposition;
        self
    }
}

impl AxumIntoResponse for FileResponse {
    fn into_response(self) -> AxumResponse {
        let disposition = match self.disposition {
            ContentDisposition::Inline => "inline",
            ContentDisposition::Attachment => "attachment",
        };

        let filename = self.filename.unwrap_or("file");
        let content_disposition = format!("{disposition}; filename=\"{filename}\"");

        let headers = [
            ("Content-Type", self.content_type),
            ("Content-Disposition", &content_disposition),
        ];

        (StatusCode::OK, headers, Body::from(self.bytes)).into_response()
    }
}
