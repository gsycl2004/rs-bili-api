
pub type BiliApiResult<T> = Result<T,BiliBiliApiError>;

#[derive(Debug)]
pub enum BiliBiliApiError{
    ReqwestError(reqwest::Error),
    ErrorCode(String,i64),
    SerdeJsonError(serde_json::Error),
    IOError(std::io::Error)
}

impl From<reqwest::Error> for BiliBiliApiError{
    fn from(value: reqwest::Error) -> Self {
        BiliBiliApiError::ReqwestError(value)
    }
}

impl From<std::io::Error> for BiliBiliApiError{
    fn from(value: std::io::Error) -> Self {
        BiliBiliApiError::IOError(value)
    }
}

impl From<serde_json::Error> for BiliBiliApiError{
    fn from(value: serde_json::Error) -> Self {
        BiliBiliApiError::SerdeJsonError(value)
    }
}