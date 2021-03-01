pub mod error;
pub mod notification;
#[cfg(feature="imgur")]
pub mod imgur;

use error::Error::{self, *};
use notification::Notification;

use serde_json::{Map, Value};
use reqwest::{Client, StatusCode};

pub async fn send<E, K>(notification: &Notification, event: E, key: K) -> Result<(), Error>
    where E: AsRef<str>, K: AsRef<str>
{
    let mut data = Map::with_capacity(3);
    data.insert("value2".to_owned(), Value::String(notification.message.to_owned()));
    if let Some(title) = notification.title.clone() {
        data.insert("value1".to_owned(), Value::String(title.to_owned()));
    }
    if let Some(extra) = notification.extra.clone() {
        data.insert("value3".to_owned(), Value::String(extra.to_owned()));
    }

    let response = Client::new()
        .post(
            &format!(
                "https://maker.ifttt.com/trigger/{}/with/key/{}",
                event.as_ref(), key.as_ref()
            )
        )
        .json(&data)
        .send().await
        .map_err(|e| InvalidIftttRequest { source: e.into() })?;

    if response.status() != StatusCode::OK {
        return Err(InvalidIftttStatusCode { code: response.status().as_u16() })
    }

    Ok(())
}