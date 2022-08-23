use reqwest;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio;
use yew::prelude::*;

pub async fn post<T,E>(url: String, object: E) -> Result<T, Error>
where
    T: DeserializeOwned,
    E: Serialize
{
    log::info!("in post method"); 
    let response = reqwest::Client::new()
        .post(url)
        .json(&object)
        .send()
        .await;

    log::info!("after post {:?}",response); 
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
    let response = reqwest::get(url).await;
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
