#[cfg(feature="imgur")]
use {
    crate::error::Error,
    std::borrow::Cow
};

/// A out-going notification. May be sent multiple times with the `nustify::send` function.
#[derive(Debug, Clone)]
pub struct Notification {
    pub(crate) title: Option<String>,
    pub(crate) message: String,
    pub(crate) extra: Option<String>,
}

impl Notification {
    /// Create a new notification with a optional title.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let notification = Notification::new(None, "Hello from Rust".to_owned());
    /// ```
    pub fn new(title: Option<String>, message: String) -> Self {
        Self {
            title,
            message,
            extra: None
        }
    }
}

/// A builder helper used to create a simple notification or a more complex one (link, image).
pub struct Builder(Notification);

impl Builder {
    /// Create a builder that will resolve to a notification with the provided content and an empty title.
    pub fn new(message: String) -> Self {
        Self(Notification::new(None, message))
    }

    /// Set the title of the notification.
    ///
    /// # Examples
    /// ```rust
    /// let notification = Builder::new("Rusty content".to_owned())
    ///     .title("Hello from Rust".to_owned())
    ///     .build();
    /// ```
    pub fn title(mut self, title: String) -> Self {
        self.0.title = Some(title);
        self
    }

    /// Set the `value3` of the notification.
    ///
    /// # Examples
    /// ```rust
    /// let notification = Builder::new("Rusty content".to_owned())
    ///     .extra("https://i.imgur.com/SFmiPRo.png".to_owned())
    ///    .build();
    /// ```
    pub fn extra(mut self, extra: String) -> Self {
        self.0.extra = Some(extra);
        self
    }

    /// Set the image URL of the notification (same as calling `extra`).
    ///
    /// # Examples
    /// ```rust
    /// let notification = Builder::new("Rusty content".to_owned())
    ///     .image_url("https://i.imgur.com/SFmiPRo.png".to_owned())
    ///     .build();
    /// ```
    pub fn image_url(self, url: String) -> Self {
        self.extra(url)
    }

    /// Set the link that the notification will follow when tapped (same as calling `extra`).
    ///
    /// # Examples
    /// ```rust
    /// let notification = Builder::new("Rusty content".to_owned())
    ///     .url("https://www.rust-lang.org".to_owned())
    ///     .build();
    /// ```
    pub fn url(self, url: String) -> Self {
        self.extra(url)
    }

    /// Upload an image to Imgur and use the URL returned by the API as the image URL.
    ///
    /// # Examples
    /// ```rust
    /// let image_data = std::fs::read("crab.png")?;
    /// let notification = Builder::new("A nice uploaded image".to_owned())
    ///     .imgur_image("MY_IMGUR_KEY", image_data).await?
    ///     .build();
    /// ```
    ///
    /// # Features
    /// This method requires the `imgur` feature.
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

    /// Build the notification using all the settings previously provided.
    pub fn build(self) -> Notification {
        self.0
    }
}