use std::arch::global_asm;
use std::collections::HashMap;

use paste::paste;
use reqwest::{Client, Request};
use serde::{Deserialize, Serialize};

use crate::{define_api_post, video};
use crate::err::{BiliApiResult, BiliBiliApiError};
use crate::err::BiliBiliApiError::ErrorCode;
use crate::internal::{RetData, Session};
use crate::video::Video;

define_api_post!(like,"https://api.bilibili.com/x/web-interface/archive/like",bvid,like,csrf);
define_api_post!(coin,"https://api.bilibili.com/x/web-interface/coin/add",bvid,multiply,select_like,csrf);

#[derive(Deserialize, Debug)]
struct LikeRet {
    code: i32,
    message: String,
    ttl: i32,
}

#[derive(Deserialize, Debug)]
pub struct CoinResult{
    pub like:bool
}

pub async fn like(session: &Session, video: &Video, like: bool) -> BiliApiResult<()> {
    let csrf = session.get_csrf();

    let result = session.client
        .execute(call_like(&session.client, video.bvid.as_str(), if like { "1" } else { "0" }, csrf.as_str()))
        .await?
        .json::<LikeRet>()
        .await?;

    if result.code == 0 {
        return Ok(());
    }
    Err(ErrorCode(result.message, result.code as i64))
}

pub async fn coin(session:&Session, video:&Video, multiply:i32, select_like:bool) -> Result<CoinResult, BiliBiliApiError> {
    let csrf = session.get_csrf();

    let result = session.client
        .execute(call_coin(&session.client,video.bvid.as_str(),multiply.to_string().as_str(),if select_like { "1" } else {"0"},csrf.as_str()))
        .await?
        .json::<RetData<CoinResult>>()
        .await?;

    if result.code == 0 {
        return Ok(result.data);
    }
    Err(ErrorCode(result.message, result.code))
}


#[cfg(test)]
mod test {
    use crate::login::qrcode::login;
    use crate::video::action::{coin, like};
    use crate::video::Video;

    #[tokio::test]
    async fn test() {
        let session = login(|x| {
            println!("{}", x);
        }).await.unwrap();
        let video = Video {
            bvid: String::from("BV1Jx411m7KB")
        };

        like(&session, &video,true).await.unwrap();
        coin(&session,&video,2,false).await.unwrap();
    }
}