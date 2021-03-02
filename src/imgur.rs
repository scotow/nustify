use crate::error::Error::{self, *};

use std::borrow::Cow;

use serde_json::Value;
use reqwest::{
    Client,
    StatusCode,
    header:: AUTHORIZATION,
    multipart::{Form, Part},
};

pub async fn upload<T: >(client_id: &str, image: T) -> Result<String, Error>
where
    T: Into<Cow<'static, [u8]>>
{
    let form = Form::new()
        .part("image", Part::bytes(image));

    let response = Client::new()
        .post("https://api.imgur.com/3/image")
        .header(AUTHORIZATION, format!("Client-ID {}", client_id))
        .multipart(form)
        .send().await
        .map_err(|e| InvalidImgurRequest { source: e.into() })?;
    if response.status() != StatusCode::OK {
        return Err(InvalidImgurStatusCode { code: response.status().as_u16() })
    }

    Ok(
        response
            .json::<Value>().await
            .map_err(|e| InvalidImgurResponse { source: e.into() })?
            .pointer("/data/link").ok_or(InvalidImgurJson)?
            .as_str().ok_or(InvalidImgurJson)?
            .to_owned()
    )
}