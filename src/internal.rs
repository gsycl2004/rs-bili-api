use std::sync::Arc;

use reqwest::Client;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use serde::{Deserialize, Serialize};

use crate::err::BiliApiResult;
use crate::err::BiliBiliApiError::ErrorCode;

#[derive(Serialize, Deserialize)]
pub(crate) struct RetData<T> {
    pub code: i64,
    pub message: String,
    #[serde(default = "default")]
    pub data: Option<T>,
}

fn default<T>()->Option<T>{None}


impl<T> From<RetData<T>> for BiliApiResult<T> {
    fn from(value: RetData<T>) -> Self {
        Err(ErrorCode(value.message, value.code))
    }
}


pub struct Session {
    pub(crate) cookie_store: Arc<CookieStoreMutex>,
    pub(crate) client: Client,
}

impl Session {
    pub(crate) fn new() -> Session {
        let p = Arc::new(CookieStoreMutex::new(CookieStore::default()));
        Session {
            cookie_store: Arc::clone(&p),
            client: Client::builder().cookie_store(true)
                .cookie_provider(Arc::clone(&p))
                .build()
                .unwrap(),
        }
    }

    pub(crate) fn get_csrf(&self) -> String{
        self.cookie_store.lock().unwrap().get("bilibili.com", "/", "bili_jct").unwrap().value().to_string()
    }
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test() {

    }
}