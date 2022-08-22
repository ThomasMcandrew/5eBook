use reqwest;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use futures::Future;


pub async fn post<T,E>(url: String, object: E) -> Result<T, Error>
where
    T: DeserializeOwned,
    E: Serialize,
{
    let response = reqwest::Client::new()
        .post(url)
        .json(&object)
        .to_async()
        .await;
    if let Ok(data) = response {
        if let Ok(repo) = data.json::<T>().await {
            Ok(repo)
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
        if let Ok(repo) = data.json::<T>().await {
            Ok(repo)
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
