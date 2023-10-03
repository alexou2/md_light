use std::fmt;
use std::num::ParseIntError;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum ApiError {
    REQWEST(reqwest::Error),
    JSON(serde_json::Error),
    StrError(String),
    Box(Box<dyn std::any::Any + Send>),
    NoMoreChapters
}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::REQWEST(_) => write!(f, "request error"),
            ApiError::JSON(_) => write!(f, "json conversion error"),
            ApiError::StrError(_) => write!(f, "error while processing strings"),
            ApiError::Box(_)=> write!(f, "unknown box error"),
            ApiError::NoMoreChapters => write!(f, "got all of the chapters for this manga")
        }
    }
}
impl std::convert::From<&str> for ApiError {
    fn from(err: &str) -> Self {
        // todo!()
        ApiError::StrError(err.to_string())
    }
}
impl std::convert::From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::REQWEST(err)
    }
}
impl std::convert::From<ParseIntError> for ApiError {
    fn from(_: ParseIntError) -> Self {
        todo!()
    }
}
impl std::convert::From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::JSON(err)
    }
}

impl std::convert::From<Box<dyn std::error::Error>> for ApiError {
    fn from(_: Box<dyn std::error::Error>) -> Self {
        todo!()
    }
}
impl std::convert::From<JoinError> for ApiError {
    fn from(_: JoinError) -> Self {
        todo!()
    }
}
impl std::convert::From<std::string::String> for ApiError {
    fn from(err: String) -> Self {
        ApiError::StrError(err)
    }
}
impl std::convert::From<Box<dyn std::any::Any + Send>> for ApiError{
    fn from(err: Box<dyn std::any::Any + Send>) -> Self {
        ApiError::Box(err)
    }
}