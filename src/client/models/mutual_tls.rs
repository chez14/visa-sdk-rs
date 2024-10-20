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
/// use visa_sdk::client::state::MutualTls;
/// use visa_sdk::api::hello_world;
///
/// let mtls = MutualTls {
///    user_id: "application-id-from-visa-sama".to_string(),
///   password: "super-secret-password-from-visa-sama".to_string()
///       cert: "--- BEGIN ...".to_string(),
///   cert_key: Some("super-secret-key".to_string()),
///  ca_bundle: "your_ca_bundle".to_string(),
/// };
///
/// let client = VisaClient::builder()
///     .set_mutual_tls(mtls)
///     .build();
///
/// let hello_world = VisaClient::hello_world(false);
/// let response = hello_world.get();
/// println!("{:?}", response);
/// ```
#[derive(Default, Clone, Debug)]
pub struct MutualTls {
    /// The user ID to use for the API Client to authenticate. This value is
    /// obtainable in your application dashboard.
    pub user_id: String,

    /// The password to use for the API Client to authenticate. This value is
    /// also obtainable in your application dashboard.
    pub password: String,

    /// The certificate to use for the client, the certificate content, not the
    /// path. Certificate should be in PKCS12 format. This certificate will be
    /// loaded by reqwest's Identity struct.
    ///
    /// See [`reqwest::Identity::from_pkcs12_der`] for more information.
    pub cert: Vec<u8>,

    /// Certificate Passphrase if any. If the certificate is not password
    /// protected, this should be [None].
    pub cert_key: Option<String>,
}
