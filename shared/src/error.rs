use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Invalid infohash.")]
    InfoHash,
}

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Database error: {0}")]
    DatabseError(String),
    #[error("Decoding error: {0}")]
    DecodingError(String),
}

pub type Result<T> = std::result::Result<T, BackendError>;

impl actix_web::ResponseError for BackendError {
    #[inline]
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        log::error!("The request generated this error: {self}");
        actix_web::HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
