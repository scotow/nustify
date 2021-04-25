use thiserror::Error;

type ErrorSource = Box<dyn std::error::Error>;

/// All possible errors returned by the crate.
#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid ifttt http request")]
    InvalidIftttRequest { source: ErrorSource },
    #[error("invalid ifttt http status code")]
    InvalidIftttStatusCode { code: u16 },
    #[cfg(feature="imgur")]
    #[error("invalid imgur http request")]
    InvalidImgurRequest { source: ErrorSource },
    #[cfg(feature="imgur")]
    #[error("invalid imgur http status code")]
    InvalidImgurStatusCode { code: u16 },
    #[cfg(feature="imgur")]
    #[error("invalid imgur response")]
    InvalidImgurResponse { source: ErrorSource },
    #[cfg(feature="imgur")]
    #[error("invalid imgur json data")]
    InvalidImgurJson,
}