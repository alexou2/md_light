use std::num::ParseIntError;
use std::fmt;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum ApiError {
    REQWEST(reqwest::Error),
    JSON(serde_json::Error),
}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::REQWEST(_) => write!(f, "Error Variant 2 occurred"),
            ApiError::JSON(_) => write!(f, "Error Variant 2 occurred"),
        }
    }
}
impl std::convert::From<&str> for ApiError {
    fn from(_: &str) -> Self {
        todo!()
    }
}
impl std::convert::From<reqwest::Error> for ApiError {
    fn from(_: reqwest::Error) -> Self {
        todo!()
    }
}
impl std::convert::From<ParseIntError> for ApiError {
    fn from(_: ParseIntError) -> Self {
        todo!()
    }
}
impl std::convert::From<serde_json::Error> for ApiError{
    fn from(_: serde_json::Error) -> Self {
        todo!()
    }
}

impl std::convert::From<Box<dyn std::error::Error>> for ApiError {
    fn from(_: Box<dyn std::error::Error>) -> Self {
        todo!()
    }
}
impl std::convert::From<JoinError> for ApiError{
    fn from(_: JoinError) -> Self {
        todo!()
    }
}
impl std::convert::From<std::string::String> for ApiError{
    fn from(_: String) -> Self {
        todo!()
    }
}