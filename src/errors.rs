use serde_json;
use std::io;

#[derive(Debug)]
pub enum OthelloError {
    Json(serde_json::Error),
    Io(io::Error),
}

impl From<io::Error> for OthelloError {
    fn from(err: io::Error) -> OthelloError {
        OthelloError::Io(err)
    }
}

impl From<serde_json::Error> for OthelloError {
    fn from(err: serde_json::Error) -> OthelloError {
        OthelloError::Json(err)
    }
}

pub type OthelloResult<T> = Result<T, OthelloError>;
