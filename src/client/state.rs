use super::{message_level_encryption::MessageLevelEncryption, mutual_tls::MutualTls};

// region:    --- Mutual TLS ---
#[derive(Default, Clone, Debug)]
pub struct WithoutMutualTls;
// region:    --- Mutual TLS ---

#[derive(Default, Clone, Debug)]
pub struct WithMutualTls(pub(crate) MutualTls);

pub trait MutualTlsState {}
impl MutualTlsState for WithoutMutualTls {}
impl MutualTlsState for WithMutualTls {}
// endregion: --- Mutual TLS ---

// region:    --- Message Level Encryption ---
#[derive(Default, Clone)]
pub struct WithoutMessageLevelEncryption;

#[derive(Default, Clone)]
pub struct WithMessageLevelEncryption(pub(crate) MessageLevelEncryption);

pub trait MessageLevelEncryptionState {}
impl MessageLevelEncryptionState for WithoutMessageLevelEncryption {}
impl MessageLevelEncryptionState for WithMessageLevelEncryption {}
#[cfg(test)]
impl MessageLevelEncryptionState for () {}
// endregion: --- Message Level Encryption ---
