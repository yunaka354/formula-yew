use gloo_net::http::Request;
use serde::Deserialize;

pub async fn fetch_server<T>(url: &str) -> T
where
    T: for<'de> Deserialize<'de>,
{
    let response: T = Request::get(url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    response
}
