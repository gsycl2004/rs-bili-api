use std::fmt::Error;
use std::sync::Arc;
use reqwest::Client;
use reqwest_cookie_store::{CookieStore, CookieStoreMutex};
use serde::{Serialize, Deserialize};
use crate::err::BiliApiResult;
use crate::err::BiliBiliApiError::ErrorCode;

#[derive(Serialize,Deserialize)]
pub(crate) struct RetData<T>{
    pub code:i64,
    pub message:String,
    pub data:T
}

impl <T> Into<BiliApiResult<T>>  for RetData<T>{
    fn into(self) -> BiliApiResult<T> {
        Err(ErrorCode(self.message,self.code))
    }
}


pub struct Session{
    pub(crate) cookie_store:Arc<CookieStoreMutex>,
    pub(crate) client:Client
}

impl Session {
    pub(crate) fn new()->Session{
        let p = Arc::new(CookieStoreMutex::new(CookieStore::default()));
        Session{
            cookie_store:Arc::clone(&p),
            client:Client::builder().cookie_store(true)
                .cookie_provider(Arc::clone(&p))
                .build()
                .unwrap(),
        }
    }

}

#[cfg(test)]
mod test{



    #[tokio::test]
    async fn test(){



    }
}