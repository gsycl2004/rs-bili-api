use std::borrow::Borrow;
use std::io::Read;

use flate2::read::GzDecoder;
use paste::paste;
use reqwest::{Method, Request, Url};
use serde::Deserialize;

use crate::define_api_get;
use crate::err::BiliApiResult;
use crate::err::BiliBiliApiError::ErrorCode;
use crate::internal::{RetData, Session};

define_api_get!(account,"https://api.bilibili.com/x/member/web/account",);
define_api_get!(reward,"https://api.bilibili.com/x/member/web/exp/reward",);
define_api_get!(exp,"https://www.bilibili.com/plus/account/exp.php",);

#[derive(Deserialize, Debug)]
pub struct AccountData {
    pub mid: i64,
    pub uname: String,
    pub userid: String,
    pub sign: String,
    pub birthday: String,
    pub sex: String,
    pub nick_free: bool,
    pub rank: String,
}

#[derive(Deserialize, Debug)]
pub struct RewardData {
    pub login: bool,
    pub watch: bool,
    pub coins: i64,
    pub share: bool,
    pub email: bool,
    pub tel: bool,
    pub safe_question: bool,
    pub identify_card: bool,
}

#[derive(Deserialize, Debug)]
pub struct DailyExp {
    pub code: i64,
    pub message: String,
    pub number: i32,
}

pub async fn account(session: &Session) -> BiliApiResult<AccountData> {
    let result = session.client
        .execute(call_account())
        .await?
        .json::<RetData<AccountData>>()
        .await?;

    if result.code == 0 {
        return Ok(result.data.unwrap());
    }
    result.into()
}

pub async fn reward(session: &Session) -> BiliApiResult<RewardData> {
    let result = session.client
        .execute(call_reward())
        .await?
        .json::<RetData<RewardData>>()
        .await?;
    if result.code == 0 {
        return Ok(result.data.unwrap());
    }
    result.into()
}

pub async fn exp(session: &Session) -> BiliApiResult<i32> {
    let result = session.client
        .execute(call_exp())
        .await?
        .bytes()
        .await?;
    let mut decoder = GzDecoder::<&[u8]>::new(result.borrow());
    let mut text = String::new();
    decoder.read_to_string(&mut text)?;
    let p = serde_json::from_str::<DailyExp>(text.as_str())?;
    if p.code == 0 {
        return Ok(p.number);
    }
    Err(ErrorCode(p.message, p.code))
}


#[cfg(test)]
mod test {
    use std::borrow::Borrow;
    use std::io::Read;

    use flate2::read::GzDecoder;
    use reqwest::{header, Method, Request};

    use crate::login::member_center::{account, exp, reward, RewardData};
    use crate::login::qrcode::{login, QRCodeHandler};

    #[test]
    fn t() {}

    #[tokio::test]
    async fn test() {
        let session = login(
            QRCodeHandler::Image(|x| {
                println!("{}", x);
            })
        ).await.unwrap();

        println!("{:?}", account(&session).await.unwrap());
        println!("{:?}", reward(&session).await.unwrap());
        println!("{:?}", exp(&session).await.unwrap());
    }
}