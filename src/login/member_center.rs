use std::borrow::Borrow;
use std::io::Read;
use flate2::read::GzDecoder;
use serde::Deserialize;
use crate::define_api;
use crate::err::BiliApiResult;
use crate::internal::{RetData, Session};
use paste::paste;
use reqwest::{Method, Request, Url};
use serde_json::Value;
use crate::err::BiliBiliApiError::ErrorCode;



define_api!(account,"https://api.bilibili.com/x/member/web/account",);
define_api!(reward,"https://api.bilibili.com/x/member/web/exp/reward",);
define_api!(exp,"https://www.bilibili.com/plus/account/exp.php",);

#[derive(Deserialize,Debug)]
pub struct AccountData{
    pub mid:i64,
    pub uname:String,
    pub userid:String,
    pub sign:String,
    pub birthday:String,
    pub sex:String,
    pub nick_free:bool,
    pub rank:String
}

#[derive(Deserialize,Debug)]
pub struct RewardData{
    login:bool,
    watch:bool,
    coins:i64,
    share:bool,
    email:bool,
    tel:bool,
    safe_question:bool,
    identify_card:bool,
}
#[derive(Deserialize,Debug)]
pub struct DailyExp{
    code:i64,
    message:String,
    number:i32,
}

pub async fn account(session:&Session) -> BiliApiResult<AccountData> {
    let result = session.client
        .execute(call_account())
        .await?
        .json::<RetData<AccountData>>()
        .await?;

    if result.code == 0 {
        return Ok(result.data)
    }
    result.into()
}

pub async fn reward(session:&Session) -> BiliApiResult<RewardData> {
    let result = session.client
        .execute(call_reward())
        .await?
        .json::<RetData<RewardData>>()
        .await?;
    if result.code == 0 {
        return Ok(result.data)
    }
    result.into()
}

pub async fn exp(session:&Session) -> BiliApiResult<i32> {
    let result = session.client
        .execute(call_exp())
        .await?
        .bytes()
        .await?;
    let mut decoder = GzDecoder::<&[u8]>::new(result.borrow());
    let mut text= String::new();
    decoder.read_to_string(&mut text)?;
    let p = serde_json::from_str::<DailyExp>(text.as_str())?;
    if p.code == 0 {
        return Ok(p.number);
    }
    Err(ErrorCode(p.message,p.code))





}


#[cfg(test)]
mod test{
    use std::borrow::Borrow;
    use std::io::Read;
    use flate2::read::GzDecoder;

    use reqwest::{header, Method, Request};


    use crate::login::member_center::{account, exp, reward, RewardData};
    use crate::login::qrcode::login;

    #[test]
    fn t(){





    }

    #[tokio::test]
    async fn test(){

        let session = login(
            |x|{
                println!("{}", x);
            }
        ).await.unwrap();

        println!("{:?}", account(&session).await.unwrap());
        println!("{:?}", reward(&session).await.unwrap());
        println!("{:?}", exp(&session).await.unwrap());
    }
}