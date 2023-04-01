use serde::Deserialize;
use crate::define_api_get;
use paste::paste;
use reqwest::{Request,Method,Url};
use crate::err::BiliApiResult;
use crate::err::BiliBiliApiError::ErrorCode;
use crate::internal::{RetData, Session};
use crate::video::Video;

define_api_get!(total,"https://api.bilibili.com/x/player/online/total",bvid,cid);

#[derive(Debug,Deserialize)]
pub struct ShowSwitch {
    pub total: bool,
    pub count: bool,
}

#[derive(Debug,Deserialize)]
pub struct Total{
    pub total: String,
    pub count: String,
    pub show_switch: ShowSwitch,
}

pub async fn total(session:&Session,video:&Video,cid:&str)->BiliApiResult<Total>{
    let result = session.client
        .execute(call_total(video.bvid.as_str(),cid))

        .await?
        .json::<RetData<Total>>()
        .await?;
    if result.code == 0 {
        return Ok(result.data.unwrap());
    }
    Err(ErrorCode(result.message, result.code))

}


#[cfg(test)]
mod test{
    use crate::login::qrcode::{login, QRCodeHandler};
    use crate::video::info::pagelist;
    use crate::video::online::total;
    use crate::video::Video;

    #[tokio::test]
    async fn test(){
        let session = login(QRCodeHandler::Image(|x| {
            println!("{}", x);
        })).await.unwrap();
        let video = Video {
            bvid: String::from("BV1tX4y1d7bj")
        };
        let cid = pagelist(&session,&video).await.unwrap()[0].cid;
        println!("{:?}",total(&session,&video,cid.to_string().as_str()).await.unwrap());
    }

}
