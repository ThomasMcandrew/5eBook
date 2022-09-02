use reqwest;
use serde::{de::DeserializeOwned,Serialize};

fn get_base_url() -> String {
    String::from("http://127.0.0.1:5000/")
}

pub async fn post<T>(url: String, object: T) -> Result<T, Error>
where
    T: DeserializeOwned + Serialize
{
    let full_url = get_base_url() + url.as_str();
    let response = reqwest::Client::new()
        .request(reqwest::Method::POST,
            full_url)
        .json(&object)
        .send()
        .await;

    if let Ok(data) = response {
        if let Ok(val) = data.json::<T>().await {
            Ok(val)
        } else {
            Err(Error::DeserializeError)
        }
    } else {
        Err(Error::RequestError)
    }
}
pub async fn get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let response = reqwest
        ::get(get_base_url() + url.as_str())
        .await;
    if let Ok(data) = response {
        if let Ok(val) = data.json::<T>().await {
            Ok(val)
        } else {
            Err(Error::DeserializeError)
        }
    } else {
        Err(Error::RequestError)
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    RequestError,
    DeserializeError,
    // etc.
}
