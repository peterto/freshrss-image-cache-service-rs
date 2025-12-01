use crate::http_error::HttpError;
use axum::body::Body;
use axum::http::{header, HeaderName, HeaderValue};
use axum::response::IntoResponse;
use std::str::FromStr;

pub struct CachedImage {
    pub data: Vec<u8>,
    pub mime_type: String,
    pub extracted_from_cache: bool,
    pub filename: String,
}

impl IntoResponse for CachedImage {
    fn into_response(self) -> axum::response::Response {
        let content_type_value = match HeaderValue::from_str(&self.mime_type) {
            Ok(value) => value,
            Err(e) => {
                let wrapped_error = HttpError::Internal(anyhow::Error::new(e));

                return wrapped_error.into_response();
            }
        };

        let mut response = Body::from(self.data).into_response();
        let headers = response.headers_mut();

        let cache_status_value = if self.extracted_from_cache {
            "HIT"
        } else {
            "MISS"
        };

        headers.insert(header::CONTENT_TYPE, content_type_value);
        headers.insert(
            HeaderName::from_str("X-Piccache-Status").unwrap(),
            HeaderValue::from_str(cache_status_value).unwrap(),
        );

        let filename_with_ext = format!(
            "{}.{}",
            self.filename,
            mime_to_extension(&self.mime_type).unwrap_or("bin")
        );

        headers.insert(
            HeaderName::from_str("Content-Disposition").unwrap(),
            HeaderValue::from_str(&format!("attachment; filename=\"{}\"", filename_with_ext))
                .unwrap(),
        );

        response
    }
}

fn mime_to_extension(mime_type: &str) -> Option<&'static str> {
    match mime_type {
        // Common image formats
        "image/jpeg" => Some("jpg"),
        "image/png" => Some("png"),
        "image/gif" => Some("gif"),
        "image/webp" => Some("webp"),
        "image/svg+xml" => Some("svg"),
        "image/bmp" => Some("bmp"),
        "image/x-icon" => Some("ico"),
        "image/vnd.microsoft.icon" => Some("ico"),
        "image/tiff" => Some("tiff"),
        "image/avif" => Some("avif"),
        "image/heic" => Some("heic"),
        "image/heif" => Some("heif"),

        // Less common but valid image formats
        "image/x-bmp" => Some("bmp"),
        "image/x-ms-bmp" => Some("bmp"),
        "image/apng" => Some("apng"),
        "image/jxl" => Some("jxl"),

        // RAW formats
        "image/x-canon-cr2" => Some("cr2"),
        "image/x-canon-crw" => Some("crw"),
        "image/x-nikon-nef" => Some("nef"),
        "image/x-sony-arw" => Some("arw"),

        _ => None,
    }
}