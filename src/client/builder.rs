use super::{
    api_level::ApiLevel, config::Config, mutual_tls::MutualTls, state::{self, WithMutualTls}, VisaClient
};

#[derive(Default, Clone)]
pub struct VisaClientBuilder<MTLS, MLE>
where
    MTLS: state::MutualTlsState + Clone,
    MLE: state::MessageLevelEncryptionState + Clone,
{
    mutual_tls: MTLS,
    message_level_encryption: MLE,
    api_level: ApiLevel,
}

impl VisaClientBuilder<state::WithoutMutualTls, state::WithoutMessageLevelEncryption> {
    pub fn new() -> Self {
        VisaClientBuilder {
            ..VisaClientBuilder::default()
        }
    }
}

/// Final state of the builder. All APIs require mutual TLS, but only some of
/// them require Message Level Encryption.
impl<MLE> VisaClientBuilder<state::WithMutualTls, MLE>
where
    MLE: state::MessageLevelEncryptionState + Clone,
{
    pub fn build(&self) -> VisaClient<MLE> {
        VisaClient {
            mutual_tls: self.mutual_tls.0.clone(),
            message_level_encryption: self.message_level_encryption.clone(),
            config: self.build_api_config(),
        }
    }
}

impl<MTLS, MLE> VisaClientBuilder<MTLS, MLE>
where
    MTLS: state::MutualTlsState + Clone,
    MLE: state::MessageLevelEncryptionState + Clone,
{
    pub fn set_mutual_tls(
        self,
        mutual_tls: MutualTls,
    ) -> VisaClientBuilder<state::WithMutualTls, MLE> {
        VisaClientBuilder {
            mutual_tls: WithMutualTls(mutual_tls),
            message_level_encryption: self.message_level_encryption,
            api_level: self.api_level,
        }
    }

    pub fn set_message_level_encryption(
        self,
        message_level_encryption: state::WithMessageLevelEncryption,
    ) -> VisaClientBuilder<MTLS, state::WithMessageLevelEncryption> {
        VisaClientBuilder {
            message_level_encryption,
            mutual_tls: self.mutual_tls,
            api_level: self.api_level,
        }
    }

    pub fn set_api_level(self, api_level: ApiLevel) -> Self {
        VisaClientBuilder { api_level, ..self }
    }

    fn build_api_config(&self) -> Config {
        Config {
            api_level: self.api_level,
        }
    }
}
