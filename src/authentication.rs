/// The ACCEPT_HEADER_VALUE is compiled as constant to compile check it.
const ACCEPT_HEADER_VALUE: reqwest::header::HeaderValue =
    reqwest::header::HeaderValue::from_static("text/plain");
/// The CONTENT_TYPE_HEADER_VALUE is compiled as constant to compile check it.
const CONTENT_TYPE_HEADER_VALUE: reqwest::header::HeaderValue =
    reqwest::header::HeaderValue::from_static("application/json");
/// The PUBLIC_API_KEY_HEADER_NAME is compiled as constant to compile check it.
const PUBLIC_API_KEY_HEADER_NAME: reqwest::header::HeaderName =
    reqwest::header::HeaderName::from_static("publicapikey");
/// The REQUEST_TOKEN_HEADER_NAME is compiled as constant to compile check it.
const REQUEST_TOKEN_HEADER_NAME: reqwest::header::HeaderName =
    reqwest::header::HeaderName::from_static("requesttoken");
/// The REQUEST_SIGNATURE_HEADER_NAME is compiled as constant to compile check it.
const REQUEST_SIGNATURE_HEADER_NAME: reqwest::header::HeaderName =
    reqwest::header::HeaderName::from_static("requestsignature");
/// The CLIENT_ID_HEADER_NAME is compiled as constant to compile check it.
const CLIENT_ID_HEADER_NAME: reqwest::header::HeaderName =
    reqwest::header::HeaderName::from_static("clientid");

impl crate::TwelveConfig {
    /// Creates a Headermap for Twelve
    /// The input should be the path of the url
    /// i.e. `/api/v1/RevenueTransactions`
    pub fn generate_headermap(
        &self,
        path: impl AsRef<str>,
    ) -> Result<reqwest::header::HeaderMap, TwelveHeadermapError> {
        // Create current time in yyyymmdd (i.e. 20261026)
        // This is used in the request token as per Twelve API documentation: https://clientapi.twelve.eu/info/authentication.html
        let date = jiff::Timestamp::try_from(std::time::SystemTime::now())
            .map_err(TwelveHeadermapError::UnknownSystemTime)?
            .to_zoned(jiff::tz::TimeZone::UTC)
            .strftime("%Y%m%d");

        // Generate a string of 30 random characters
        let randomstring = rand::distr::SampleString::sample_string(
            &rand::distr::Alphanumeric,
            &mut rand::rng(),
            30,
        );

        // Combine data and randomstring to get a request key in accordance with the Twelve api.
        let requesttoken = format!("{date}{randomstring}");

        // Call the sign functions on the private_key to create a request signature
        let requestsignature = self.private_key().sign(path.as_ref(), &requesttoken);

        // Create a HeaderMap
        let mut headermap = reqwest::header::HeaderMap::new();
        headermap.insert(reqwest::header::ACCEPT, ACCEPT_HEADER_VALUE);
        headermap.insert(reqwest::header::CONTENT_TYPE, CONTENT_TYPE_HEADER_VALUE);

        let mut public_key = reqwest::header::HeaderValue::from_str(self.public_key())
            .map_err(TwelveHeadermapError::ParseErrorPublicKey)?;
        public_key.set_sensitive(true);
        headermap.insert(PUBLIC_API_KEY_HEADER_NAME, public_key);

        let mut requesttoken = reqwest::header::HeaderValue::from_str(&requesttoken)
            .map_err(TwelveHeadermapError::ParseErrorPublicKey)?;
        requesttoken.set_sensitive(true);
        headermap.insert(REQUEST_TOKEN_HEADER_NAME, requesttoken);

        let mut requestsignature = reqwest::header::HeaderValue::from_str(&requestsignature)
            .map_err(TwelveHeadermapError::ParseErrorPublicKey)?;
        requestsignature.set_sensitive(true);
        headermap.insert(REQUEST_SIGNATURE_HEADER_NAME, requestsignature);

        headermap.insert(
            CLIENT_ID_HEADER_NAME,
            self.client_id()
                .parse()
                .map_err(TwelveHeadermapError::ParseErrorClientId)?,
        );

        Ok(headermap)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TwelveHeadermapError {
    #[error("Could not create a timestamp")]
    UnknownSystemTime(#[source] jiff::Error),
    #[error("Could not parse the PublicKey")]
    ParseErrorPublicKey(#[source] reqwest::header::InvalidHeaderValue),
    #[error("Could not parse the Request Token")]
    ParseErrorRequestToken(#[source] reqwest::header::InvalidHeaderValue),
    #[error("Could not parse the Request Signature")]
    ParseErrorRequestSignature(#[source] reqwest::header::InvalidHeaderValue),
    #[error("Could not parse the ClientId")]
    ParseErrorClientId(#[source] reqwest::header::InvalidHeaderValue),
}
