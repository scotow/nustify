use crate::error::Error::{self, *};

use isahc::{
    http::Request,
    prelude::*,
    http::header::{AUTHORIZATION, CONTENT_TYPE}
};
use serde_json::Value;
use common_multipart_rfc7578::client::multipart::{Form, Body};
use futures::TryStreamExt;

pub async fn upload(client_id: &str, image: &[u8]) -> Result<String, Error> {
    let mut form = Form::default();
    form.add_reader("image", image);
    let boundary = form.content_type();

    let data = Body::from(form)
        .try_concat().await
        .map_err(|e| InvalidRequest { source: e.into() })?
        .freeze();

    let mut response = Request::post("https://api.imgur.com/3/image")
        .header(AUTHORIZATION, format!("Client-ID {}", client_id))
        .header(CONTENT_TYPE, boundary)
        .body(data.as_ref()).map_err(|e| InvalidRequest { source: e.into() })?
        .send_async().await.map_err(|e| ImgurApi { source: e.into() })?;

    let data = response.text().await.map_err(|e| ImgurApi { source: e.into() })?;
    let data = serde_json::from_str::<Value>(&data).map_err(|e| Json { source: e.into() })?;
    Ok(
        data.as_object().ok_or(ImgurJson)?
            .get("data").ok_or(ImgurJson)?.as_object().ok_or(ImgurJson)?
            .get("link").ok_or(ImgurJson)?.as_str().ok_or(ImgurJson)?
            .to_string()
    )
}