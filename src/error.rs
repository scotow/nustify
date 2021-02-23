use thiserror::Error;

type ErrorSource = Box<dyn std::error::Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot build json data")]
    Json { source: ErrorSource },
    #[error("invalid http request")]
    InvalidRequest { source: ErrorSource },
    #[error("ifttt api call error")]
    IftttApi { source: ErrorSource },
    #[cfg(feature="imgur")]
    #[error("image is too large for imgur")]
    ImageTooLarge,
    #[cfg(feature="imgur")]
    #[error("imgur api call error")]
    ImgurApi { source: ErrorSource },
    #[cfg(feature="imgur")]
    #[error("cannot deserialize imgur's json")]
    ImgurJson,
}