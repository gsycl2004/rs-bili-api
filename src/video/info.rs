use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use crate::define_api_get;
use crate::err::BiliApiResult;
use crate::internal::{RetData, Session};
use paste::paste;
use reqwest::{Request,Method,Url};
use crate::video::Video;
define_api_get!(view,"https://api.bilibili.com/x/web-interface/view",bvid);

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct VideoInfo{
    pub bvid: String,
    pub aid: i64,
    pub videos: i8,
    pub tid: i32,
    pub tname: String,
    pub copyright: i8,
    pub pic: String,
    pub title: String,
    pub pubdate: i64,
    pub ctime: i64,
    pub desc: String,
    pub desc_v2: Vec<DescV2>,
    pub state: i32,
    pub duration: i64,
    #[serde(default = "default")]
    pub forward:Option<i32>,
    pub mission_id: i32,
    #[serde(default = "default")]
    pub redirect_url:Option<String>,
    pub rights: Rights,
    pub owner: Owner,
    pub stat: Stat,
    pub dynamic: String,
    pub cid: i64,
    pub dimension: Dimension,
    pub premiere: Value,
    pub teenage_mode: i8,
    pub is_chargeable_season: bool,
    pub is_story: bool,
    pub no_cache: bool,
    pub pages: Vec<Page>,
    pub subtitle: Subtitle,
    #[serde(default = "default")]
    pub staff: Option<Vec<Staff>>,
    pub is_season_display: bool,
    pub user_garb: UserGarb,
    pub honor_reply: HonorReply,
    pub like_icon: String,
}

fn default<T>()->Option<T>{None}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescV2 {
    pub raw_text: String,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub biz_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Rights {
    pub bp: i8,
    pub elec: i8,
    pub download: i8,
    pub movie: i8,
    pub pay: i8,
    pub hd5: i8,
    pub no_reprint: i8,
    pub autoplay: i8,
    pub ugc_pay: i8,
    pub is_cooperation: i8,
    pub ugc_pay_preview: i32,
    pub no_background: i32,
    pub clean_mode: i32,
    pub is_stein_gate: i8,
    pub is_360: i8,
    pub no_share: i32,
    pub arc_pay: i32,
    pub free_watch: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Owner {
    pub mid: i32,
    pub name: String,
    pub face: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Stat {
    pub aid: i64,
    pub view: i64,
    pub danmaku: i64,
    pub reply: i64,
    pub favorite: i64,
    pub coin: i64,
    pub share: i64,
    pub now_rank: i64,
    pub his_rank: i64,
    pub like: i64,
    pub dislike: i64,
    pub evaluation: String,
    pub argue_msg: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Dimension {
    pub width: i64,
    pub height: i64,
    pub rotate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Page {
    pub cid: i64,
    pub page: i64,
    pub from: String,
    pub part: String,
    pub duration: i64,
    pub vid: String,
    pub weblink: String,
    pub dimension: Dimension2,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Dimension2 {
    pub width: i64,
    pub height: i64,
    pub rotate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Subtitle {
    pub allow_submit: bool,
    pub list: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Staff {
    pub mid: i64,
    pub title: String,
    pub name: String,
    pub face: String,
    pub vip: Vip,
    pub official: Official,
    pub follower: i64,
    pub label_style: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Vip {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub status: i64,
    pub due_date: i64,
    pub vip_pay_type: i64,
    pub theme_type: i64,
    pub label: Label,
    pub avatar_subscript: i64,
    pub nickname_color: String,
    pub role: i64,
    pub avatar_subscript_url: String,
    pub tv_vip_status: i64,
    pub tv_vip_pay_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Label {
    pub path: String,
    pub text: String,
    pub label_theme: String,
    pub text_color: String,
    pub bg_style: i64,
    pub bg_color: String,
    pub border_color: String,
    pub use_img_label: bool,
    pub img_label_uri_hans: String,
    pub img_label_uri_hant: String,
    pub img_label_uri_hans_static: String,
    pub img_label_uri_hant_static: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Official {
    pub role: i64,
    pub title: String,
    pub desc: String,
    #[serde(rename = "type")]
    pub type_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UserGarb {
    pub url_image_ani_cut: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct HonorReply {
    pub honor: Vec<Honor>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Honor {
    pub aid: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub desc: String,
    pub weekly_recommend_num: i64,
}

pub async fn view(session:&Session,video:&Video)->BiliApiResult<VideoInfo>{
    let data = session.client
        .execute(call_view(video.bvid.as_str()))
        .await?
        .json::<RetData<VideoInfo>>()
        .await?;

    if data.code == 0 {
        return Ok(data.data.unwrap());
    }
    data.into()

}

#[cfg(test)]
mod test {
    use crate::login::qrcode::{login, QRCodeHandler};
    use crate::video::action::{coin, deal, like, triple};
    use crate::video::info::{view, Vip};
    use crate::video::Video;

    #[tokio::test]
    async fn test_abcd() {
        let session = login(QRCodeHandler::Image(|x| {
            println!("{}", x);
        })).await.unwrap();
        let video = Video {
            bvid: String::from("BV1tX4y1d7bj")
        };

        println!("{:?}", view(&session, &video).await.unwrap());

    }
}