extern crate reqwest;
use reqwest::header;
#[test]
fn test() -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    //headers.insert("authority", "api.bilibili.com".parse().unwrap());
    //headers.insert("accept", "application/json, text/plain, */*".parse().unwrap());
    //headers.insert("accept-language", "zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6".parse().unwrap());
    headers.insert("cache-control", "no-cache".parse().unwrap());
    headers.insert("content-type", "application/x-www-form-urlencoded".parse().unwrap());
    headers.insert(header::COOKIE, "SESSDATA=4ff5b902%2C1695817362%2C5e20e%2A31".parse().unwrap());
    //headers.insert("origin", "https://www.bilibili.com".parse().unwrap());
    //headers.insert("pragma", "no-cache".parse().unwrap());
    //headers.insert("referer", "https://www.bilibili.com/video/BV1Ps4y1n7r8/?spm_id_from=444.41.list.card_archive.click&vd_source=7417b6c496f9a2079d44f3d758cbb9ef".parse().unwrap());
    //headers.insert("sec-ch-ua", "\"Microsoft Edge\";v=\"111\", \"Not(A:Brand\";v=\"8\", \"Chromium\";v=\"111\"".parse().unwrap());
    //headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    //headers.insert("sec-ch-ua-platform", "\"Windows\"".parse().unwrap());
    //headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    //headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    //headers.insert("sec-fetch-site", "same-site".parse().unwrap());
    //headers.insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36 Edg/111.0.1661.54".parse().unwrap());

    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client.post("https://api.bilibili.com/x/web-interface/archive/like/triple")
        .headers(headers)
        .body("bvid=BV1FL411S7RU&csrf=6ccc5400f7cbab030d8493d4fe49cdea&eab_x=2&ramval=1&source=web_normal&ga=1")
        .send()?
        .text()?;
    println!("{}", res);

    Ok(())
}