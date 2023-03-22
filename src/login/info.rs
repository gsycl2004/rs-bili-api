use crate::err::BiliApiResult;

use crate::internal::{RetData, Session};
use serde::{Serialize,Deserialize};
///[详细信息](https://github.com/gsycl2004/bilibili-API-collect/blob/master/docs/login/login_info.md)


#[derive(Serialize,Deserialize,Debug)]
pub struct Nav {
    pub isLogin: bool,
    pub email_verified: i64,
    pub face: String,
    pub level_info: LevelInfo,
    pub mid: i64,
    pub mobile_verified: i64,
    pub money: f64,
    pub moral: i64,
    pub official: Official,
    pub pendant: Pendant,
    pub scores: i64,
    pub uname: String,
    pub vipDueDate: i64,
    pub vipStatus: i8,
    pub vipType: i8,
    pub vip_pay_type: i8,
    pub vip_theme_type: i64,
    pub vip_label: VipLabel,
    pub vip_avatar_subscript: i8,
    pub vip_nickname_color: String,
    pub wallet:Wallet,
    pub has_shop:bool,
    pub shop_url:String,
    pub allowance_count:i32,
    pub answer_status:i32
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Wallet {
    pub mid: i64,
    pub bcoin_balance: i64,
    pub coupon_balance: i64,
    pub coupon_due_time: i64,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct VipLabel {
    pub path: String,
    pub text: String,
    pub label_theme: String,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Pendant {
    pub pid: i64,
    pub name: String,
    pub image: String,
    pub expire: i64,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Official {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: i64,
    pub desc: String,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct LevelInfo {
    pub current_level: i64,
    pub current_min: i64,
    pub current_exp: i64,
    pub next_exp: i64,
}
///# 获取导航栏用户信息
///[详细信息](https://github.com/gsycl2004/bilibili-API-collect/blob/master/docs/login/login_info.md#%E5%AF%BC%E8%88%AA%E6%A0%8F%E7%94%A8%E6%88%B7%E4%BF%A1%E6%81%AF)
async fn nav(session: &Session) -> BiliApiResult<Nav> {
    let request = session
        .client
        .get("https://api.bilibili.com/x/web-interface/nav")
        .build()?;
    let resp = session
        .client
        .execute(request)
        .await?
        .json::<RetData<Nav>>()
        .await?;
    if resp.code == 0 {
        return Ok(resp.data);
    }
    resp.into()
}

#[cfg(test)]
mod test{

    use crate::login::info::nav;
    use crate::login::qrcode::login;

    #[tokio::test]
    async fn test(){
        let session = login(|x|{
            println!("{}",x);
        }).await.unwrap();
        let t = nav(&session).await.unwrap();
        println!("{:?}",t);
    }
}