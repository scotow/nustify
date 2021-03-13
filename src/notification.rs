#[cfg(feature="imgur")]
use crate::error::Error;

#[cfg(feature="imgur")]
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Notification {
    pub(crate) title: Option<String>,
    pub(crate) message: String,
    pub(crate) extra: Option<String>,
}

impl Notification {
    pub fn new(title: Option<String>, message: String) -> Self {
        Self {
            title,
            message,
            extra: None
        }
    }
}

pub struct Builder(Notification);

impl Builder {
    pub fn new(message: String) -> Self {
        Self(Notification::new(None, message))
    }

    pub fn title(mut self, title: String) -> Self {
        self.0.title = Some(title);
        self
    }

    pub fn extra(mut self, extra: String) -> Self {
        self.0.extra = Some(extra);
        self
    }

    pub fn image_url(self, url: String) -> Self {
        self.extra(url)
    }

    pub fn url(self, url: String) -> Self {
        self.extra(url)
    }

    #[cfg(feature="imgur")]
    pub async fn imgur_image<T>(self, client_id: &str, image: T) -> Result<Self, Error>
    where
        T: Into<Cow<'static, [u8]>>
    {
        Ok(
            self.image_url(
                crate::imgur::upload(client_id, image).await?
            )
        )
    }

    pub fn build(self) -> Notification {
        self.0
    }
}