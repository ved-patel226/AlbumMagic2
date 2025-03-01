use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Item not found: {0}")]
    NotFound(String),
}
