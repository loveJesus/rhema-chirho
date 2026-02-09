// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! API error types.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

/// API error type that converts to HTTP responses.
#[derive(Debug, Error)]
pub enum ApiErrorChirho {
    #[error("Not found: {message_chirho}")]
    NotFoundChirho { message_chirho: String },

    #[error("Bad request: {message_chirho}")]
    BadRequestChirho { message_chirho: String },

    #[error("Internal server error: {message_chirho}")]
    InternalChirho { message_chirho: String },
}

impl IntoResponse for ApiErrorChirho {
    fn into_response(self) -> Response {
        let (status_chirho, body_chirho) = match &self {
            ApiErrorChirho::NotFoundChirho { message_chirho } => {
                (StatusCode::NOT_FOUND, message_chirho.clone())
            }
            ApiErrorChirho::BadRequestChirho { message_chirho } => {
                (StatusCode::BAD_REQUEST, message_chirho.clone())
            }
            ApiErrorChirho::InternalChirho { message_chirho } => {
                (StatusCode::INTERNAL_SERVER_ERROR, message_chirho.clone())
            }
        };
        (status_chirho, body_chirho).into_response()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_not_found_error_chirho() {
        let err_chirho = ApiErrorChirho::NotFoundChirho {
            message_chirho: "Module KJV not found".to_string(),
        };
        assert_eq!(err_chirho.to_string(), "Not found: Module KJV not found");
    }

    #[test]
    fn test_bad_request_error_chirho() {
        let err_chirho = ApiErrorChirho::BadRequestChirho {
            message_chirho: "Invalid query".to_string(),
        };
        assert_eq!(err_chirho.to_string(), "Bad request: Invalid query");
    }
}
