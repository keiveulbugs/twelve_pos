/// The `TwelveClient` struct is used to access all API calls easily
///  API calls are implementations of this structure.
///
/// # Example
/// ```
/// # fn main() -> Result<(), twelve_pos::TwelveConfigError> {
/// let client = twelve_pos::TwelveClient::builder()
///     .client_id("1234")
///     .public_key("public_key")
///     .private_key("private_key")
///     .url("https://clientapi.twelve.eu")
///     .build()?;
/// # Ok(()) }
/// ```
#[derive(Clone)]
pub struct TwelveClient {
    // Available through `.config()` so that users don't accidentally change values.
    twelve_config: std::sync::Arc<crate::config_builder::TwelveConfig>,
    // Available in the crate
    pub(crate) network_client: reqwest::Client,
}

#[bon::bon]
impl TwelveClient {
    /// # Example
    /// ```
    /// # fn main() -> Result<(), twelve_pos::TwelveConfigError> {
    /// let client = twelve_pos::TwelveClient::builder()
    ///     .client_id("1234")
    ///     .public_key("public_key")
    ///     .private_key("private_key")
    ///     .url("https://clientapi.twelve.eu")
    ///     .build()?;
    /// # Ok(()) }
    /// ```
    #[builder(
        builder_type(vis = "pub"),
        start_fn(name = builder, vis = "pub"),
        finish_fn(vis = "pub"),
        on(String, into)
    )]
    fn new(
        /// Twelve ClientID.
        /// Avoid adding a zero infront of the ClientID unless it is specifically part of your ClientID.
        /// This crate will automatically add a zero infront of the ClientID when required.
        client_id: String,
        /// Twelve PublicKey.
        public_key: String,
        /// Twelve PrivateKey. Wrapped the moment the setter is called, so it is
        /// zeroized on drop even if `build()` is never reached.
        #[builder(with = |private_key: impl Into<String>| crate::privatekey::PrivateKey::new(private_key))]
        private_key: crate::privatekey::PrivateKey,
        /// Allows using a custom API URL instead of the default <https://clientapi.twelve.eu>.
        #[builder(setters(option_fn(vis = "")))]
        url: Option<String>,
    ) -> Result<Self, crate::config_builder::TwelveConfigError> {
        let config =
            crate::config_builder::TwelveConfig::new(client_id, public_key, private_key, url)?;

        Ok(Self {
            twelve_config: std::sync::Arc::new(config),
            network_client: reqwest::Client::new(),
        })
    }

    /// Read-only view of the configuration this client was built with.
    pub(crate) fn config(&self) -> &crate::config_builder::TwelveConfig {
        &self.twelve_config
    }
}

impl std::fmt::Debug for TwelveClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TwelveClient")
            .field("TwelveConfig", &self.twelve_config)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn builder_test() {
        let client = crate::TwelveClient::builder()
            .client_id("1234")
            .public_key("public_key")
            .private_key("private_key")
            .url("https://clientapi.twelve.eu")
            .build();
        assert!(client.is_ok(), "Test if client can be constructed.")
    }
}
