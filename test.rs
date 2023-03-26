use std::collections::HashMap;
use reqwest::{Client, Method, Request, RequestBuilder};

#[tokio::test]
async fn test_rs() {
    let client = Client::new();
    let mut map = HashMap::new();
    map.insert("text","hello world");
    let mut b = client.post("").form(&map).build().unwrap();
}