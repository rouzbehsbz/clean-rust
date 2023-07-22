use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Oops! Something went wrong.")]
    InternalServerError,
}