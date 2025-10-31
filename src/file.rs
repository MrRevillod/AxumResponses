use axum::{http::StatusCode, response::IntoResponse};

use crate::http::HttpResponse;

pub type FileResult = Result<FileResponse, HttpResponse>;

#[derive(Default, Debug)]
pub struct FileResponse {
    bytes: Option<Vec<u8>>,
    filename: String,
    content_type: String,
    disposition: ContentDisposition,
}

#[derive(Debug, Default)]
pub enum ContentDisposition {
    Inline,
    #[default]
    Attachment,
}

impl FileResponse {
    pub fn builder() -> FileResponse {
        FileResponse::default()
    }

    pub fn bytes(mut self, bytes: Vec<u8>) -> Self {
        self.bytes = Some(bytes);
        self
    }

    pub fn filename(mut self, filename: &str) -> Self {
        self.filename = filename.to_string();
        self
    }

    pub fn disposition(mut self, disposition: ContentDisposition) -> Self {
        self.disposition = disposition;
        self
    }

    pub fn content_type(mut self, content_type: &str) -> Self {
        self.content_type = content_type.to_string();
        self
    }

    #[doc(hidden)]
    pub fn build(self) -> impl IntoResponse {
        let bytes = self.bytes.unwrap_or_default();
        let disposition = match self.disposition {
            ContentDisposition::Inline => "inline",
            ContentDisposition::Attachment => "attachment",
        };

        let filename = if self.filename.is_empty() {
            "file"
        } else {
            &self.filename
        };

        (
            StatusCode::OK,
            [
                ("Content-type", self.content_type.as_str()),
                (
                    "Content-disposition",
                    &format!("{disposition}; filename=\"{filename}\""),
                ),
            ],
            bytes,
        )
            .into_response()
    }
}

impl IntoResponse for FileResponse {
    fn into_response(self) -> axum::response::Response {
        self.build().into_response()
    }
}
