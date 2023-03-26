use std::collections::HashMap;
use paste::paste;
use reqwest::{Client, Request};
use serde::Deserialize;
use crate::define_api_post;
use crate::err::{BiliApiResult, BiliBiliApiError};
use crate::err::BiliBiliApiError::ErrorCode;
use crate::internal::{RetData, Session};
use crate::video::Video;

define_api_post!(like,"https://api.bilibili.com/x/web-interface/archive/like",bvid,like,csrf);
define_api_post!(coin,"https://api.bilibili.com/x/web-interface/coin/add",bvid,multiply,select_like,csrf);
define_api_post!(deal,"https://api.bilibili.com/medialist/gateway/coll/resource/deal",rid,r#type,add_media_ids,del_media_ids,csrf);
define_api_post!(triple,"https://api.bilibili.com/x/web-interface/archive/like/triple",bvid,csrf);



#[derive(Deserialize, Debug)]
struct LikeRet {
    code: i32,
    message: String,
    ttl: i32,
}

#[derive(Deserialize, Debug)]
pub struct CoinResult {
    pub like: bool,
}

#[derive(Deserialize, Debug)]
pub struct PromptResult {
    pub prompt: bool,
}

#[derive(Deserialize, Debug)]
pub struct TripleResult{
    pub like:bool,
    pub coin:bool,
    pub fav:bool,
    pub multiply:i32
}


pub async fn like(session: &Session, video: &Video, like: i8) -> BiliApiResult<()> {
    let csrf = session.get_csrf();

    let result = session.client
        .execute(call_like(&session.client, video.bvid.as_str(), like.to_string().as_str(), csrf.as_str()))
        .await?
        .json::<LikeRet>()
        .await?;

    if result.code == 0 {
        return Ok(());
    }
    Err(ErrorCode(result.message, result.code as i64))
}

pub async fn coin(session: &Session, video: &Video, multiply: i32, select_like: bool) -> Result<CoinResult, BiliBiliApiError> {
    let csrf = session.get_csrf();
    let result = session.client
        .execute(call_coin(&session.client, video.bvid.as_str(), multiply.to_string().as_str(), if select_like { "1" } else { "0" }, csrf.as_str()))
        .await?
        .json::<RetData<CoinResult>>()
        .await?;
    if result.code == 0 {
        return Ok(result.data.unwrap());
    }
    Err(ErrorCode(result.message, result.code))
}

pub async fn deal(session: &Session, video: &Video, add_media_ids: Option<i32>, del_media_ids: Option<i32>) -> Result<PromptResult, BiliBiliApiError> {
    let csrf = session.get_csrf();
    let p;
    let r;
    let result = session.client
        .execute(call_deal(&session.client, video.to_aid().to_string().as_str(), "2", match add_media_ids {
            None => "",
            Some(v) => {
                p = v.to_string();
                p.as_str()
            }
        }, match del_media_ids {
            None => "",
            Some(v) => {
                r = v.to_string();
                r.as_str()
            }
        }, csrf.as_str()))
        .await?
        .json::<RetData<PromptResult>>()
        .await?;
    if result.code == 0 {
        return Ok(result.data.unwrap());
    }
    Err(ErrorCode(result.message, result.code))
}

pub async fn triple(session:&Session,video:&Video)->BiliApiResult<TripleResult>{
    let csrf = session.get_csrf();
    let result = session.client
        .execute(call_triple(&session.client, video.bvid.as_str(),csrf.as_str()))

        .await?
        .json::<RetData<TripleResult>>()
        .await?;
    if result.code == 0 {
        return Ok(result.data.unwrap());
    }
    Err(ErrorCode(result.message, result.code))

}


#[cfg(test)]
mod test {
    use crate::login::qrcode::login;
    use crate::video::action::{coin, deal, like, triple};
    use crate::video::Video;

    #[tokio::test]
    async fn test() {
        let session = login(|x| {
            println!("{}", x);
        }).await.unwrap();
        let video = Video {
            bvid: String::from("BV1hD4y1X7Rm")
        };

        like(&session, &video, 2).await;
        coin(&session, &video, 2, false).await;
        deal(&session, &video, Some(106709609), None).await;
        triple(&session,&video).await.unwrap();
    }
}