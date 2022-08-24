use reqwest;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio;
use yew::prelude::*;

use crate::login::LoginModel;

pub async fn post<T>(url: String, object: LoginModel) -> Result<T, Error>
where
    T: DeserializeOwned
{
    log::info!("in post method {:?}", object); 
    
    let response = reqwest::Client::new()
        .post(url)
        .fetch_mode_no_cors()
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
