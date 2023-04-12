

use serde::Deserialize;
use serde_json::Value;

use crate::{define_api_get, err::BiliApiResult, internal::{RetData, Session}};
use super::Live;

define_api_get!(get_info,"https://api.live.bilibili.com/room/v1/Room/get_info",room_id);
define_api_get!(room_init,"https://api.live.bilibili.com/room/v1/Room/room_init",id);



#[derive(Debug,Deserialize)]
pub struct LiveRet{
    pub uid: i64,
    pub room_id: i64,
    pub short_id: i64,
    pub attention: i64,
    pub online: i64,
    pub is_portrait: bool,
    pub description: String,
    pub live_status: i64,
    pub area_id: i64,
    pub parent_area_id: i64,
    pub parent_area_name: String,
    pub old_area_id: i64,
    pub background: String,
    pub title: String,
    pub user_cover: String,
    pub keyframe: String,
    pub is_strict_room: bool,
    pub live_time: String,
    pub tags: String,
    pub is_anchor: i64,
    pub room_silent_type: String,
    pub room_silent_level: i64,
    pub room_silent_second: i64,
    pub area_name: String,
    pub pendants: String,
    pub area_pendants: String,
    pub hot_words: Vec<String>,
    pub hot_words_status: i64,
    pub verify: String,
    pub new_pendants: NewPendants,
    pub up_session: String,
    pub pk_status: i64,
    pub pk_id: i64,
    pub battle_id: i64,
    pub allow_change_area_time: i64,
    pub allow_upload_cover_time: i64,
    pub studio_info: StudioInfo,
}


#[derive(Debug,Deserialize)]
pub struct NewPendants {
    pub frame: Frame,
    pub badge: Badge,
    pub mobile_frame: MobileFrame,
    pub mobile_badge: Value,
}

#[derive(Debug,Deserialize)]
pub struct Frame {
    pub name: String,
    pub value: String,
    pub position: i64,
    pub desc: String,
    pub area: i64,
    pub area_old: i64,
    pub bg_color: String,
    pub bg_pic: String,
    pub use_old_area: bool,
}

#[derive(Debug,Deserialize)]
pub struct Badge {
    pub name: String,
    pub position: i64,
    pub value: String,
    pub desc: String,
}

#[derive(Debug,Deserialize)]
pub struct MobileFrame {
    pub name: String,
    pub value: String,
    pub position: i64,
    pub desc: String,
    pub area: i64,
    pub area_old: i64,
    pub bg_color: String,
    pub bg_pic: String,
    pub use_old_area: bool,
}

#[derive(Debug,Deserialize)]
pub struct StudioInfo {
    pub status: i64,
    pub master_list: Vec<Value>,
}

#[derive(Debug,Deserialize)]
pub struct RoomInitRet{
    pub room_id: i64,
    pub short_id: i64,
    pub uid: i64,
    pub need_p2p: i64,
    pub is_hidden: bool,
    pub is_locked: bool,
    pub is_portrait: bool,
    pub live_status: i64,
    pub hidden_till: i64,
    pub lock_till: i64,
    pub encrypted: bool,
    pub pwd_verified: bool,
    pub live_time: i64,
    pub room_shield: i64,
    pub is_sp: i64,
    pub special_type: i64,
}



pub async fn get_info(session:&Session,live:&Live) -> BiliApiResult<LiveRet>{
    let result = session.client
        .execute(call_get_info(&live.room_id.to_string()))
        .await?
        .json::<RetData<LiveRet>>()
        .await?;

    if result.code == 0 {
        return Ok(result.data.unwrap());
    }
    result.into()
}

pub async fn room_init(session:&Session,live:&Live) -> BiliApiResult<RoomInitRet>{
    let result = session.client
        .execute(call_get_info(&live.room_id.to_string()))
        .await?
        .json::<RetData<RoomInitRet>>()
        .await?;

    if result.code == 0 {
        return Ok(result.data.unwrap());
    }
    result.into()
}

#[cfg(test)]
mod test{
    use crate::{live::Live, internal::Session};
    use crate::live::info::get_info;

    #[tokio::test]
    async fn test(){
        let live = Live::from_room_id(1);
        let session = Session::raw();
        let result = get_info(&session, &live).await.unwrap();
        println!("{:?}",result);
    }
}