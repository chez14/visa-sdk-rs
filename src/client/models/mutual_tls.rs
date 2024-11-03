use derive_builder::Builder;

/// Mutual TLS is required by all APIs, as mentioned in the Visa API
/// Documentation.
///
/// Important notes: you will need to provide your own CA Bundle for this.
/// Please check Visa's documentation for more information. You may also use the
/// systemn CA bundle.
///
/// Please see the following links for more information:
/// - <https://developer.visa.com/pages/working-with-visa-apis/visa-developer-quick-start-guide#section5>
///
/// This crate will load the certificate using reqwest's Identity struct in
/// PKCS12 format. Please consult with
/// [reqwest's documentation][reqwest::Identity::from_pkcs12_der] for more information.
///
/// ## Example
/// ```no_run
/// use visa_sdk::client::{VisaClientBuilder, models::MutualTlsBuilder};
/// use visa_sdk::api::hello_world;
///
/// let mtls = MutualTlsBuilder::default()
///     .user_id("application-id-from-visa")
///     .password("super-secret-password-from-visa")
///     .cert("--- BEGIN ...".as_bytes().to_vec())
///     .cert_key(Some(String::from("super-secret-key")))
///     .build()
///     .unwrap();
///
/// let client = VisaClientBuilder::new()
///     .set_mutual_tls(mtls)
///     .build();
/// ```
#[derive(Default, Clone, Debug, Builder)]
#[builder(build_fn(error = "crate::utils::BuilderError"))]
pub struct MutualTls {
    /// The user ID to use for the API Client to authenticate. This value is
    /// obtainable in your application dashboard.
    #[builder(setter(into))]
    pub(crate) user_id: String,

    /// The password to use for the API Client to authenticate. This value is
    /// also obtainable in your application dashboard.
    #[builder(setter(into))]
    pub(crate) password: String,

    /// The certificate to use for the client, the certificate content, not the
    /// path. Certificate should be in PKCS12 format. This certificate will be
    /// loaded by reqwest's Identity struct.
    ///
    /// See [`reqwest::Identity::from_pkcs12_der`] for more information.
    pub(crate) cert: Vec<u8>,

    /// Certificate Passphrase if any. If the certificate is not password
    /// protected, this should be [None].
    #[builder(setter(into))]
    pub(crate) cert_key: Option<String>,
}
