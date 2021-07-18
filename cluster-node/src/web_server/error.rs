use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Current thread doesn't have system")]
    ThreadDoesntHaveSystem,
    #[error("BindError Error({0:?}): '{0}'")]
    BindError(
        #[source]
        #[from]
        std::io::Error,
    ),
}
