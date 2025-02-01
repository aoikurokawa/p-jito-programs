use error::JitoRestakingApiError;

pub mod error;
pub mod router;
pub mod state;

pub type Result<T> = std::result::Result<T, JitoRestakingApiError>;
