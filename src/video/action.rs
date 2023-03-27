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
define_api_post!(share,"https://api.bilibili.com/x/web-interface/share/add",bvid,csrf);


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

///[detail](https://github.com/gsycl2004/bilibili-API-collect/blob/master/docs/video/action.md#%E6%8A%95%E5%B8%81%E8%A7%86%E9%A2%91web%E7%AB%AF)
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
///[detail](https://github.com/gsycl2004/bilibili-API-collect/blob/master/docs/video/action.md#%E6%8A%95%E5%B8%81%E8%A7%86%E9%A2%91web%E7%AB%AF)
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
///[detail](https://github.com/gsycl2004/bilibili-API-collect/blob/master/docs/video/action.md#%E6%94%B6%E8%97%8F%E8%A7%86%E9%A2%91%E5%8F%8C%E7%AB%AF)
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

///[detail](https://github.com/gsycl2004/bilibili-API-collect/blob/master/docs/video/action.md#%E4%B8%80%E9%94%AE%E4%B8%89%E8%BF%9E%E8%A7%86%E9%A2%91app%E7%AB%AF)
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
///[detail](https://api.bilibili.com/x/web-interface/share/add)
pub async fn share(session:&Session, video:&Video) -> Result<i64, BiliBiliApiError> {
    let csrf = session.get_csrf();
    let result = session.client
        .execute(call_share(&session.client, video.bvid.as_str(), csrf.as_str()))
        .await?
        .json::<RetData<i64>>()
        .await?;
    if result.code == 0 {
        return Ok(result.data.unwrap());
    }
    Err(ErrorCode(result.message, result.code))
}


#[cfg(test)]
mod test {
    use crate::login::qrcode::login;
    use crate::video::action::{coin, deal, like, share, triple};
    use crate::video::Video;

    #[tokio::test]
    async fn test() {
        let session = login(|x| {
            println!("{}", x);
        }).await.unwrap();
        let video = Video {
            bvid: String::from("BV1TV4y197Ux")
        };


        share(&session,&video).await.unwrap();
    }
}