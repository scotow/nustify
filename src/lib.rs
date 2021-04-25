//! # Nustify
//! 
//! [![crates.io](https://img.shields.io/crates/v/nustify.svg)](https://crates.io/crates/nustify)
//! [![Documentation](https://docs.rs/nustify/badge.svg)](https://docs.rs/nustify)
//! 
//! 💬 Send iOS/Android notifications using IFTTT's Webhook 💬
//! 
//! ```
//! nustify = "0.2"
//! ```
//! 
//! 
//! ## IFTTT
//! 
//! From Wikipedia:
//! 
//! *[IFTTT](https://ifttt.com/) is a free web-based service to create chains of simple conditional statements, called applets. An applet is triggered by changes that occur within other web services such as Gmail, Facebook, Telegram, Instagram, or Pinterest.*
//! 
//! IFTTT proposes hundreds of triggers, but the one that Notigo uses is the [Webhook](https://ifttt.com/maker_webhooks) trigger (also known as Maker Event).
//! 
//! By creating an IFTTT applet that sends a rich notification to your device when a Webhook is triggered, we can create a simple wrapper and use it in our Rust code.
//! 
//! 
//! ## IFTTT account and mobile app
//! 
//! In order to receive a notification from IFTTT, you have to create an IFTTT [account](https://ifttt.com/join) and download the [iOS](https://itunes.apple.com/us/app/ifttt/id660944635?mt=8) app or the [Android](https://play.google.com/store/apps/details?id=com.ifttt.ifttt&hl=en) app.
//! 
//! 
//! ## Creating the IFTTT applet
//! 
//! Next, you need to create the corresponding applet in your IFTTT account. Applets that use Webhook as a trigger can't be share like other applets, so you need to create it manually:
//! 
//! * Go to the applet [creation](https://ifttt.com/create) page;
//! * Search for `webhook` and select the `Receive a web request` trigger;
//! * Specify the name of the event
//! * Click on `Create trigger`;
//! * For the `that` action, search for `notification` and select the `Send a rich notification from the IFTTT app` action;
//! * Use the `Add ingredient` button to add `value1` as a title and `value2` as a message. `value3` as a link or image URL.
//! 
//! The final configuration of the applet looks like this:
//! 
//! <img width="480" alt="Screen Shot 2021-04-26 at 00 54 32" src="https://user-images.githubusercontent.com/7684550/116015612-2e4c5300-a63a-11eb-8f7f-132e5c8cedb1.png">
//! <img width="480" alt="Screen Shot 2021-04-26 at 00 55 54" src="https://user-images.githubusercontent.com/7684550/116015614-2f7d8000-a63a-11eb-9800-78b067897d99.png">
//! 
//! 
//! ## Getting the Webhook key
//! 
//! The last step before using the applet is to get your Webhook key. Head to the [Webhook settings page](https://ifttt.com/maker_webhooks) then click on the `Documentation` button on the upper right corner.
//! 
//! Now that you have created the applet and got your Webhook key, you can use the library or the example command.
//! 
//! 
//! ## Using the library
//! 
//! Here is a simple example that send a notification with "Hello from Rust" as a title and "Rusty content" as a message:
//! 
//! ```rust
//! use std::error::Error;
//! use nustify::notification::Builder;
//! 
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let key = "MY_IFTTT_KEY";
//!     let notification = Builder::new("Rusty content".to_owned())
//!         .title("Hello from Rust".to_owned())
//!         .build();
//!     nustify::send(&notification, "nustify", &key).await?;
//!     Ok(())
//! }
//! ```
//! 
//! ### Sending images
//! 
//! If you want to add an image to your notification, you have to pass its link rather than the image data itself. This library also provides a wrapper to the Imgur API allowing you to first upload the image to Imgur.

mod error;
mod notification;
#[cfg(feature="imgur")]
mod imgur;

pub use error::Error;
pub use notification::*;

use error::Error::*;

use serde_json::{Map, Value};
use reqwest::{Client, StatusCode};


/// Send a notification asynchronously.
///
/// # Examples
///
/// ```rust
/// let notification = Notification::new(None, "Hello from Rust".to_owned());
/// nustify::send(&notification, "nustify", "IFTTT_KEY").await?;
/// ```
pub async fn send(notification: &Notification, event: &str, key: &str) -> Result<(), Error> {
    let mut data = Map::with_capacity(3);
    data.insert("value2".to_owned(), Value::String(notification.message.to_owned()));
    if let Some(title) = notification.title.clone() {
        data.insert("value1".to_owned(), Value::String(title.to_owned()));
    }
    if let Some(extra) = notification.extra.clone() {
        data.insert("value3".to_owned(), Value::String(extra.to_owned()));
    }

    let response =
        Client::new()
            .post(&format!("https://maker.ifttt.com/trigger/{}/with/key/{}", event, key))
            .json(&data)
            .send().await
            .map_err(|e| InvalidIftttRequest { source: e.into() })?;
    if response.status() != StatusCode::OK {
        return Err(InvalidIftttStatusCode { code: response.status().as_u16() })
    }

    Ok(())
}