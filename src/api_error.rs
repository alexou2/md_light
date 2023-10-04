use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ApiError {
    REQWEST(reqwest::Error),
    JSON(serde_json::Error),
    StrError(String),
    Box(Box<dyn std::any::Any + Send>),
    NoMoreChapters,
    ParseIntError(ParseIntError),
}
impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::REQWEST(_) => write!(f, "Unable to make a request to the api. Please check your connection"),
            ApiError::JSON(_) => write!(f, "json conversion error"),
            ApiError::StrError(_) => write!(f, "error while processing strings"),
            ApiError::Box(_) => write!(f, "unknown box error"),
            ApiError::NoMoreChapters => write!(f, "got all of the chapters for this manga"),
            ApiError::ParseIntError(_) => write!(f, "ParseInt error"),
        }
    }
}
impl std::convert::From<&str> for ApiError {
    fn from(err: &str) -> Self {
        ApiError::StrError(err.to_string())
    }
}
impl std::convert::From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::REQWEST(err)
    }
}

impl std::convert::From<ParseIntError> for ApiError {
    fn from(err: ParseIntError) -> Self {
        ApiError::ParseIntError(err)
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
impl std::convert::From<std::string::String> for ApiError {
    fn from(err: String) -> Self {
        ApiError::StrError(err)
    }
}
impl std::convert::From<Box<dyn std::any::Any + Send>> for ApiError {
    fn from(err: Box<dyn std::any::Any + Send>) -> Self {
        ApiError::Box(err)
    }
}
