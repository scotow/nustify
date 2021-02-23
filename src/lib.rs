pub mod notification;
pub mod error;
#[cfg(feature="imgur")]
pub mod imgur;

use notification::Notification;
use error::Error::{self, *};

use isahc::{
    prelude::*,
    http::Request,
    http::header::CONTENT_TYPE
};
use serde_json::{Map, Value};

pub async fn send<E, K>(notification: &Notification, event: E, key: K) -> Result<(), Error>
where E: AsRef<str>, K: AsRef<str> {
    let mut data = Map::with_capacity(3);
    data.insert("value2".to_owned(), Value::String(notification.message.to_owned()));
    if let Some(title) = notification.title.clone() {
        data.insert("value1".to_owned(), Value::String(title.to_owned()));
    }
    if let Some(extra) = notification.extra.clone() {
        data.insert("value3".to_owned(), Value::String(extra.to_owned()));
    }
    let data = serde_json::to_vec(&data).map_err(|e| Json { source: e.into() })?;

    Request::post(format!("https://maker.ifttt.com/trigger/{}/with/key/{}", event.as_ref(), key.as_ref()))
        .header(CONTENT_TYPE, "application/json")
        .body(data).map_err(|e| InvalidRequest { source: e.into() })?
        .send_async().await.map_err(|e| IftttApi { source: e.into() })?;
    Ok(())
}