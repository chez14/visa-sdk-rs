//! # Client State Module
//!
//! This module defines the state management for the client, particularly
//! focusing on the states of Mutual TLS and Message Level Encryption (MLE). It
//! provides structures and traits to represent whether these security features
//! are enabled or not.
//!
//! ## Overview
//!
//! The module is structured around two main sets of states:
//!
//! - **Mutual TLS States**: Structures and traits that define whether Mutual
//!   TLS is enabled or not.
//! - **Message Level Encryption States**: Structures and traits that define
//!   whether Message Level Encryption is enabled or not.
//!
//! ### Available States
//!
//! #### Mutual TLS States
//! - [`WithoutMutualTls`]: Represents the state where Mutual TLS is not
//!       enabled. You cannot construct [`VisaClient`] with this state, you need
//!       to provide the mutual tls.
//! - [`WithMutualTls`]: Represents the state where Mutual TLS is enabled.
//!
//! #### Message Level Encryption States
//! - [`WithoutMessageLevelEncryption`]: Represents the state where Message
//!       Level Encryption is not enabled.
//! - [`WithMessageLevelEncryption`]: Represents the state where Message Level
//!       Encryption is enabled.

#![allow(unused_imports)] // for documentation purposes.
use super::VisaClient;
use super::{message_level_encryption::MessageLevelEncryption, mutual_tls::MutualTls};

// region:    --- Mutual TLS ---
/// Represents the state where Mutual TLS is not enabled.
#[derive(Default, Clone, Debug)]
pub struct WithoutMutualTls;

/// Represents the state where Mutual TLS is enabled.
#[derive(Default, Clone, Debug)]
pub struct WithMutualTls(pub(crate) MutualTls);

/// Trait representing the state of Mutual TLS.
pub trait MutualTlsState {}

/// Implementation of `MutualTlsState` for `WithoutMutualTls`.
impl MutualTlsState for WithoutMutualTls {}

/// Implementation of `MutualTlsState` for `WithMutualTls`.
impl MutualTlsState for WithMutualTls {}
// endregion:    --- Mutual TLS ---

// region:    --- Message Level Encryption ---
/// Represents the state where Message Level Encryption is not enabled.
#[derive(Default, Clone)]
pub struct WithoutMessageLevelEncryption;

/// Represents the state where Message Level Encryption is enabled.
#[derive(Default, Clone)]
pub struct WithMessageLevelEncryption(pub(crate) MessageLevelEncryption);

/// Trait representing the state of Message Level Encryption.
pub trait MessageLevelEncryptionState {
    /// Checks if Message Level Encryption is enabled. This one is used to help
    /// the `VisaClient` to determine if it should apply Message Level
    /// Encryption to the request.
    fn has_mle(&self) -> bool {
        false
    }
}

/// Implementation of `MessageLevelEncryptionState` for
/// `WithoutMessageLevelEncryption`.
impl MessageLevelEncryptionState for WithoutMessageLevelEncryption {}

/// Implementation of `MessageLevelEncryptionState` for
/// `WithMessageLevelEncryption`.
impl MessageLevelEncryptionState for WithMessageLevelEncryption {
    fn has_mle(&self) -> bool {
        true
    }
}

#[cfg(test)]
impl MessageLevelEncryptionState for () {}
// endregion:    --- Message Level Encryption ---
