use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("RequestError Error({0:?}): '{0}'")]
    RequestError(
        #[source]
        #[from]
        reqwest::Error,
    ),
    #[error("UrlPraseError Error({0:?}): '{0}'")]
    UrlPraseError(
        #[source]
        #[from]
        url::ParseError,
    ),
    #[error("HealthCheckError ({status_code:?}) (message:?)")]
    HealthCheckError{
        status_code: StatusCode,
        message: String
    },
}
