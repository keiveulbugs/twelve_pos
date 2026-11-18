use sha2::Digest;

/// Structure that contains the PrivateKey, seperated from the other credentials.
/// This emphasises that it needs to be secret.
#[derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
pub(crate) struct PrivateKey {
    private_key: String,
}

/// A custom Debug is implemented for the PrivateKey so that we do not accidentally leak the key in the logs.
impl std::fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(r#""**********""#)
    }
}

/// This crate uses implementations to construct and get the privatekey.
impl PrivateKey {
    /// Constructs the struct `PrivateKey`.
    pub(crate) fn new(private_key: impl Into<String>) -> Self {
        Self {
            private_key: private_key.into(),
        }
    }

    /// Gets the PrivateKey from the struct.
    /// Note: Retrieving the private key removes it from the Zeroize context.
    pub(crate) fn get_private_key(&self) -> &str {
        &self.private_key
    }

    /// Sign a RequestSignature for HeaderMap.
    pub(crate) fn sign(&self, path: &str, requesttoken: &str) -> String {
        // Generate a SHA256 hash from path+requestkey+privatekey for Twelve RequestSignature
        let requestsignaturebuilder = sha2::Sha256::new()
            .chain_update(path.as_bytes())
            .chain_update(requesttoken.as_bytes())
            .chain_update(self.private_key.as_bytes())
            .finalize();
        hex::encode_upper(requestsignaturebuilder)
    }
}

#[cfg(test)]
mod tests {

    /// Tests if the privatekey struct can be constructed from a &str and a String.
    #[test]
    fn construct_private_key() {
        let private_key: &str = "privatekey";

        // Checks if a privatekey can be constructed from a &str
        let privatekeystruct_from_str = crate::privatekey::PrivateKey::new(private_key);
        assert_eq!(
            privatekeystruct_from_str.get_private_key(),
            private_key,
            "Tests if a privatekey struct can be generated from a &str."
        );

        // Checks if a privatekey can be constructed from a String
        let privatekeystruct_from_string =
            crate::privatekey::PrivateKey::new(String::from(private_key));
        assert_eq!(
            privatekeystruct_from_string.get_private_key(),
            private_key,
            "Tests if a privatekey struct can be generated from a String."
        );
    }

    /// Tests if fmt impl does not expose the privatekey.
    #[test]
    fn key_redacted_in_output() {
        let private_key: &str = "privatekey";
        let privatekey_struct = crate::privatekey::PrivateKey::new(private_key);

        let formatted = format!("{:?}", privatekey_struct);
        assert!(!formatted.contains(private_key));
    }

    /// Tests the creation of a requestsignature
    #[test]
    fn test_signature_creation() {
        let private_key: &str = "privatekey";
        let privatekey_struct = crate::privatekey::PrivateKey::new(private_key);
        let signature = privatekey_struct.sign("path", "requesttoken");
        assert_eq!(
            &signature, "FD4BFE98E17673079EF91D91F5C38B65BBFDC648935A5B0CE66810F9274802DF",
            "Test signature creation."
        );
    }
}
