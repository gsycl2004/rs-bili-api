use std::borrow::Borrow;
use std::time::Duration;

use paste::paste;
use qrcode::QrCode;
use reqwest::{Method, Request, Url};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::{define_api_get};
use crate::err::BiliApiResult;
use crate::err::BiliBiliApiError::ErrorCode;
use crate::internal::{RetData, Session};
use crate::login::qrcode::PollEnum::{Expire, Success, UnConfirmed, UnScanned};

#[derive(Deserialize, Serialize)]
pub struct LoginQRCode {
    url: String,
    qrcode_key: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginPoll {
    url: String,
    refresh_token: String,
    timestamp: i64,
    code: i32,
    message: String,
}

pub enum PollEnum {
    Success(LoginPoll, Session),
    Expire(LoginPoll),
    UnScanned(LoginPoll),
    UnConfirmed(LoginPoll),
}

define_api_get!(poll,"https://passport.bilibili.com/x/passport-login/web/qrcode/poll",qrcode_key);
pub(crate) fn encode(url: impl AsRef<[u8]>) -> String {
    let qrcode = QrCode::new(url).unwrap();
    return qrcode.render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)

        .build();
}


pub async fn login(qrcode_handler: fn(text: &String)) -> BiliApiResult<Session> {
    let qr = generate().await;
    let LoginQRCode { url, qrcode_key } = qr.unwrap();
    qrcode_handler(&encode(&url));
    loop {
        if let Success(_poll, session) = poll(&qrcode_key).await.unwrap() {
            return Ok(session);
        }
        sleep(Duration::from_secs(1)).await;
    }
}

pub async fn poll(qrcode_key: impl Into<&String>) -> BiliApiResult<PollEnum> {
    let session = Session::new();
    let req = call_poll(qrcode_key.into());
    let resp = session.client.execute(req).await?.json::<RetData<LoginPoll>>().await?;
    if resp.code == 0 {
        let resp  = resp.data.unwrap();
        return Ok(match resp.code {
            0 => {
                Success(resp, session)
            }
            86038 => {
                Expire(resp)
            }
            86090 => {
                UnConfirmed(resp)
            }
            86101 => {
                UnScanned(resp)
            }
            _ => {
                UnConfirmed(resp)
            }
        });
    }
    Err(ErrorCode(resp.message, resp.code))
}

pub async fn generate() -> BiliApiResult<LoginQRCode> {
    let p = reqwest::get("https://passport.bilibili.com/x/passport-login/web/qrcode/generate")
        .await?.json::<RetData<LoginQRCode>>().await?;
    if p.code == 0 {
        return Ok(p.data.unwrap());
    }
    p.into()
}


#[cfg(test)]
mod test {
    use crate::login::qrcode::{generate, login, LoginQRCode, poll, PollEnum};

    #[tokio::test]
    async fn test() {
        let p = login(|x| { println!("{}", x) }).await.unwrap();
        println!("{:?}", p.cookie_store);
    }
}
