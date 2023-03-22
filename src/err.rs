pub type BiliApiResult<T> = Result<T,BiliBiliApiError>;

#[derive(Debug)]
pub enum BiliBiliApiError{
    ReqwestError(reqwest::Error),
    ErrorCode(String,i64),
    SerdeJsonError(serde_json::Error),
}

impl From<reqwest::Error> for BiliBiliApiError{
    fn from(value: reqwest::Error) -> Self {
        BiliBiliApiError::ReqwestError(value)
    }
}

impl From<serde_json::Error> for BiliBiliApiError{
    fn from(value: serde_json::Error) -> Self {
        BiliBiliApiError::SerdeJsonError(value)
    }
}