//! # Foreign Exchange API Module
//!
//! This module provides access to the Visa Foreign Exchange API, allowing users
//! to fetch real-time and indicative exchange rates for various currency pairs.
//! The module includes structures and methods to create and send requests, as
//! well as to handle responses from the API.
//!
//! ## Overview
//!
//! Visa supports both regular and enhanced versions of the Foreign Exchange
//! Rate API. Currently, this module implements only the regular version of the
//! API.
//!
//! - The regular version does not require the use of Message Level Encryption
//!   (MLE).
//! - The enhanced version requires Message Level Encryption (MLE) for added
//!   security.
//!
//! The [`ForeignExchange`] struct is the main entry point for interacting with
//! the Visa Foreign Exchange API. It provides methods to create requests for
//! different rate products, such as card-based, account-based, bank, and wallet
//! rates.
//!
//! ## Example
//!
//! Below is an example of how to create a request to fetch foreign exchange
//! rates using the [`ForeignExchange`] struct:
//!
//! ```no_run
//! use visa_sdk::client::VisaClient;
//! use visa_sdk::api::foreign_exchange::ForeignExchange;
//! use visa_sdk::api::foreign_exchange::models::{FXRequestAorBBuilder, FXRequestBankOrWalletBuilder};
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = VisaClient::new("api_key", "secret_key");
//!     let forex = ForeignExchange::new(client);
//!
//!     // Example for FXRequestAorB
//!     let payload_a_or_b = FXRequestAorBBuilder::default()
//!         .source_currency_code("USD".to_string())
//!         .destination_currency_code("GBP".to_string())
//!         .source_amount("100.55".to_string())
//!         .build()
//!         .expect("Failed to build FXRequestAorB");
//!     let response_a_or_b = forex.get_a_or_b(payload_a_or_b).await;
//!     println!("{:?}", response_a_or_b);
//!
//!     // Example for FXRequestBankOrWallet
//!     let payload_bank_or_wallet = FXRequestBankOrWalletBuilder::default()
//!         .source_currency_code("USD".to_string())
//!         .destination_currency_code("GBP".to_string())
//!         .initiating_party_id(1002)
//!         .source_amount(Some(100.55))
//!         .quote_id_required(Some(true))
//!         .build()
//!         .expect("Failed to build FXRequestBankOrWallet");
//!     let response_bank_or_wallet = forex.get_bank_or_wallet(payload_bank_or_wallet).await;
//!     println!("{:?}", response_bank_or_wallet);
//! }
//! ```
//!
//! ## Documentation
//!
//! For more details, consult the Visa API documentation:
//! - [Visa Foreign Exchange
//!   API](https://developer.visa.com/capabilities/foreign_exchange)
//! - [Visa Foreign Exchange API
//!   Reference](https://developer.visa.com/capabilities/foreign_exchange/reference)

mod regular;

pub use regular::*;
