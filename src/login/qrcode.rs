use std::borrow::Borrow;
use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;
use fast_qr::convert::image::ImageBuilder;
use fast_qr::convert::{Builder, Shape};

use fast_qr::QRBuilder;
use paste::paste;
use reqwest::{Method, Request, Url};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use uuid::Uuid;

use crate::define_api_get;
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

pub enum QRCodeHandler{
    Image(fn(img_name:&str)),
    Text(fn(text_handler:&str))
}

define_api_get!(poll,"https://passport.bilibili.com/x/passport-login/web/qrcode/poll",qrcode_key);
pub(crate) fn encode_to_text(url: impl AsRef<str>) -> String {
    let qrcode = QRBuilder::new(url.as_ref()).build().unwrap();
    qrcode.to_str()
}

pub(crate) fn encode_to_image(url: impl AsRef<str>, filename:impl AsRef<Path>){
    let qrcode = QRBuilder::new(url.as_ref()).build().unwrap();
    let _img = ImageBuilder::default()
        .shape(Shape::RoundedSquare)
        .background_color([255, 255, 255, 0]) // Handles transparency
        .fit_width(600)
        .to_file(&qrcode,filename.as_ref().to_str().unwrap());

}


fn random_name() -> String {
    Uuid::new_v4().to_string() + ".png"
}

pub async fn login(handler: QRCodeHandler) -> BiliApiResult<Session> {
    let qr = generate().await;
    let LoginQRCode { url, qrcode_key } = qr.unwrap();
    match handler {
        QRCodeHandler::Image(img) => {
            let name = random_name();
            std::fs::create_dir("tmp");

            let mut path_buf = PathBuf::new();
            path_buf.push("tmp");
            path_buf.push(&name);
            encode_to_image(url,path_buf.as_path());
            img(name.as_str())
        },
        QRCodeHandler::Text(text) => text(encode_to_text(url).as_str())
    }
    loop {
        if let Success(_poll, session) = poll(&qrcode_key).await.unwrap() {
            return Ok(session);
        }
        sleep(Duration::from_secs(1)).await;
    }
}

pub async fn login_with_img(qrcode_handler: fn(text: &String)) -> BiliApiResult<Session> {
    let qr = generate().await;
    let LoginQRCode { url, qrcode_key } = qr.unwrap();
    qrcode_handler(&encode_to_text(&url));
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
        let resp = resp.data.unwrap();
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
    use crate::login::qrcode::{generate, login, LoginQRCode, poll, PollEnum, QRCodeHandler};

    #[tokio::test]
    async fn test() {
        let p = login(QRCodeHandler::Image(|x|{
            println!("{}", x);
        })).await.unwrap();
        println!("{:?}", p.cookie_store);
    }
}
