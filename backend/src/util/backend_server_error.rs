use actix_web::{HttpResponse, ResponseError};

//these are just used on teh edge for actix_web::Result, to print out errors to the http client when theres a bad actix request
#[derive(thiserror::Error, Debug)]
pub enum BackendServerError {
    #[error("An unknown error occurred")]
    UnknownError,

    #[error("Database Insert Error {0:?}")]
    DatabaseInsertError(Option<String>),

    #[error("Database Record Not Found")]
    DatabaseRecordNotFound,

    #[error("Input Parsing Error")]
    InputParsingError,

    #[error("Unsupported Currency")]
    UnsupportedCurrency,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Image Upload Error")]
    ImageUploadError,

    #[error("Server Config Error")]
    ServerConfigError, // Add other error variants as needed
}

impl ResponseError for BackendServerError {
    fn error_response(&self) -> HttpResponse {
        match &self {
            BackendServerError::UnknownError => {
                HttpResponse::InternalServerError().json("An unknown error occurred")
            }
            BackendServerError::UnsupportedCurrency => {
                HttpResponse::InternalServerError().json("Unsupported Currency")
            }
            BackendServerError::ImageUploadError => {
                HttpResponse::InternalServerError().json("Image Upload Error")
            }
            BackendServerError::InputParsingError => {
                HttpResponse::InternalServerError().json("Input Parsing Error")
            }
            BackendServerError::Unauthorized => {
                HttpResponse::InternalServerError().json("Unauthorized")
            }
            BackendServerError::ServerConfigError => {
                HttpResponse::InternalServerError().json("Server Config Error")
            }
            BackendServerError::DatabaseInsertError(message) => HttpResponse::InternalServerError()
                .json(format!(
                    "Database Insert Error: {}",
                    message.clone().unwrap_or("Unknown".into())
                )),
            BackendServerError::DatabaseRecordNotFound => {
                HttpResponse::InternalServerError().json("Database record not found")
            }
        }
    }
}
