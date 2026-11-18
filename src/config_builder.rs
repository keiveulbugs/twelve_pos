/// The default value to communicate with the Twelve API.
/// More info can be found here https://clientapi.twelve.eu/index.html.
const DEFAULT_TWELVE_URL: &str = "https://clientapi.twelve.eu";

/// This structure contains the credentials for Twelve.
pub struct TwelveConfig {
    /// Twelve ClientID.
    /// Avoid adding a zero infront of the ClientID unless it is specifically part of your ClientID.
    /// This crate will automatically add a zero infront of the ClientID when required.
    client_id: String,
    /// Twelve PublicKey
    public_key: String,
    /// Twelve PrivateKey
    private_key: crate::privatekey::PrivateKey,
    /// Allows using a custom API URL instead of the default https://clientapi.twelve.eu.
    url: reqwest::Url,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TwelveConfigError {
    #[error("ClientId is missing or empty")]
    MissingClientId,
    #[error("ClientId can only be a positive number")]
    InvalidClientId,
    #[error("PublicKey is missing or empty")]
    MissingPublicKey,
    #[error("PrivateKey is missing or empty")]
    MissingPrivateKey,
    #[error("Custom URL is invalid")]
    InvalidCustomURL,
}
impl TwelveConfig {
    /// Validates the inputs. Only reachable from inside the crate, so an
    /// unvalidated config can never exist.
    pub(crate) fn new(
        client_id: String,
        public_key: String,
        private_key: crate::privatekey::PrivateKey,
        url: Option<String>,
    ) -> Result<Self, TwelveConfigError> {
        if client_id.is_empty() {
            return Err(TwelveConfigError::MissingClientId);
        }
        if client_id.parse::<u16>().is_err() {
            return Err(TwelveConfigError::InvalidClientId);
        }
        if public_key.is_empty() {
            return Err(TwelveConfigError::MissingPublicKey);
        }
        if private_key.get_private_key().is_empty() {
            return Err(TwelveConfigError::MissingPrivateKey);
        }
        let url = url
            .as_deref()
            .unwrap_or(DEFAULT_TWELVE_URL)
            .parse::<reqwest::Url>()
            .map_err(|_| TwelveConfigError::InvalidCustomURL)?;

        Ok(Self {
            client_id,
            public_key,
            private_key,
            url,
        })
    }

    /// Twelve ClientID.
    pub fn client_id(&self) -> &str {
        &self.client_id
    }

    /// Twelve PublicKey.
    pub fn public_key(&self) -> &str {
        &self.public_key
    }

    /// The API URL in use (the default unless a custom one was given).
    pub fn url(&self) -> &reqwest::Url {
        &self.url
    }

    /// Deliberately not public: the private key never leaves the crate.
    pub(crate) fn private_key(&self) -> &crate::privatekey::PrivateKey {
        &self.private_key
    }
}

impl std::fmt::Debug for TwelveConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TwelveConfig")
            .field("client_id", &self.client_id)
            .field("public_key", &self.public_key)
            .field("private_key", &self.private_key) // prints <redacted>
            .field("url", &self.url)
            .finish()
    }
}
