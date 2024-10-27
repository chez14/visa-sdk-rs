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

    pub(crate) _client: reqwest::Client,
}

impl<MLE> VisaClient<MLE>
where
    MLE: state::MessageLevelEncryptionState + Clone,
{
    fn apply_auth(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req.basic_auth(
            self.mutual_tls.user_id.clone(),
            Some(self.mutual_tls.password.clone()),
        )
    }

    fn apply_message_level_encryption(
        &self,
        req: reqwest::RequestBuilder,
    ) -> reqwest::RequestBuilder {
        if !self.message_level_encryption.has_mle() {
            return req;
        }

        // TODO: implement this
        req
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    /// Executes a request with the given `reqwest::Request` object. This
    /// function will apply the necessary authentication and message level
    /// encryption to the request before sending it.
    pub async fn execute_request(
        &self,
        request: reqwest::Request,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let builder = reqwest::RequestBuilder::from_parts(self._client.clone(), request);
        let authed_request = self.apply_auth(builder);
        let mle_request = self.apply_message_level_encryption(authed_request);
        mle_request.send().await
    }
}

#[cfg(test)]
mock! {
    pub VisaClient<MLE> {
      pub fn get_config(&self) -> &Config;
      pub async fn execute_request(
        &self,
        request: reqwest::Request,
    ) -> Result<reqwest::Response, reqwest::Error>  ;
    }

    impl<MLE> Clone for VisaClient<MLE> {
        fn clone(&self) -> Self;
    }
}
