pub mod delete;
pub mod get;
pub mod patch;
pub mod post;

pub use get::GetRequestBuilder;

pub struct Callers<'a> {
    twelveclient: &'a crate::TwelveClient,
}

impl crate::TwelveClient {
    pub fn callers(&self) -> Callers<'_> {
        Callers { twelveclient: self }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("The Headermap has exceeded capacity")]
    ExceededCapacity,
    #[error("Sending additional headers with the same name as existing headers mandated by Twelve is disallowed to prevent accidentally leaking secrets.:\n
    Supplied Invalid Header: {0}\n
    Not allowed headers: `accept`, `PublicAPIKey`, `RequestToken`, `RequestSignature`, `ClientId`")]
    InvalidHeaders(String),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Headermap(#[from] crate::authentication::TwelveHeadermapError),
    #[error("Could not serialize the input")]
    Serialization(#[from] serde_json::Error),
}
