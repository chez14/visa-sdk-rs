use super::{
    config::Config,
    mutual_tls::MutualTls,
    state::{self},
};
#[cfg(test)]
use mockall::mock;

// TODO: build documentation for this. TODO: make sure you add an example on how
// to make a new object in this. Also explain the type states also.
#[derive(Debug, Clone)]
pub struct VisaClient<MLE>
where
    MLE: state::MessageLevelEncryptionState + Clone,
{
    pub(super) mutual_tls: MutualTls,
    pub(super) message_level_encryption: MLE,
    pub(crate) config: Config,
}

impl<MLE> VisaClient<MLE>
where
    MLE: state::MessageLevelEncryptionState + Clone,
{
    pub fn build_reqwest(&self) -> reqwest::Client {
        // TODO: remove unwrap.
        // TODO: think about the MLE thingy.

        let certificate_identity = reqwest::Identity::from_pkcs12_der(
            &self.mutual_tls.cert,
            match &self.mutual_tls.cert_key {
                Some(cert_key) => cert_key,
                None => "",
            },
        )
        .unwrap();

        reqwest::Client::builder()
            .identity(certificate_identity)
            .default_headers(Self::build_headers())
            .build()
            .unwrap()
    }

    fn build_headers() -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();

        let version = env!("CARGO_PKG_VERSION");
        let user_agent = format!("visa_sdk/v{}", version);

        headers.append(
            reqwest::header::USER_AGENT,
            reqwest::header::HeaderValue::from_str(user_agent.as_str()).unwrap(),
        );

        headers
    }

    pub fn apply_auth(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req.basic_auth(
            self.mutual_tls.user_id.clone(),
            Some(self.mutual_tls.password.clone()),
        )
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mock! {
    pub VisaClient<MLE> {
      pub fn build_reqwest(&self) -> reqwest::Client;
      pub(crate) fn apply_auth(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
      pub fn get_config(&self) -> &Config;
    }

    impl<MLE> Clone for VisaClient<MLE> {
        fn clone(&self) -> Self;
    }
}
