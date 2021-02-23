use crate::error::Error;

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
    pub async fn imgur_image(self, client_id: &str, image: &[u8]) -> Result<Self, Error> {
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