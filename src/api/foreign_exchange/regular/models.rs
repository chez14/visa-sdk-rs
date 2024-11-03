//! # Foreign Exchange Rates Models
//!
//! This module contains data models used to interact with the Visa Foreign
//! Exchange Rates API, allowing users to request real-time and indicative
//! exchange rates for various currency pairs. The models represent different
//! request and response formats based on the type of rate product specified,
//! such as card-based, account-based, or wallet-based rates.
//!
//! ## Overview
//!
//! The module is structured around two main sets of models:
//!
//! - **Requests**: Structures that define the input parameters for querying
//!   exchange rates, tailored for different rate products (`A`, `B`, `BANK`,
//!   and `WALLET`).
//! - **Responses**: Structures that capture the expected response formats based
//!   on the type of rate product queried.
//!
//! ### Available Models
//!
//! #### Request Models
//! - [`FXRequestAorB`]: For rate products `A` and `B`, representing card- and
//!   account-based rates with options for markup and acquirer details.
//! - [`FXRequestBankOrWallet`]: For rate products `BANK` and `WALLET`,
//!   supporting real-time or quote-based exchange rates for account and wallet
//!   transactions, respectively.
//!
//! #### Response Models
//! - [`FXResponseAorB`]: Represents the exchange rate and resulting amounts
//!   when querying for rate products `A` or `B`.
//! - [`FXResponseBankOrWallet`]: Returns real-time or quoted exchange rates for
//!   `BANK` and `WALLET` products, along with optional quote IDs and expiration
//!   times.
//!
//! ## Usage
//!
//! To create a request, you will typically use the builder pattern to construct
//! the desired request model. Each model includes a `Builder` derived from the
//! `derive_builder` crate, which simplifies constructing requests by setting
//! optional parameters as needed.
//!
//! ### Example: Building a Request for Rate Product `BANK`
//!
//! ```rust
//! use your_module::FXRequestBankOrWalletBuilder;
//!
//! let request = FXRequestBankOrWalletBuilder::default()
//!     .source_currency_code("USD".to_string())
//!     .destination_currency_code("GBP".to_string())
//!     .initiating_party_id(1002)
//!     .source_amount(Some(100.55))
//!     .quote_id_required(Some(true))
//!     .build()
//!     .expect("Failed to build FXRequestBankOrWallet");
//! ```
//!
//! The example above demonstrates creating a `BANK` rate request using
//! [`FXRequestBankOrWallet`]. Using `Builder` ensures that all required fields
//! are set, while optional fields can be added as needed.
//!
//! ## General Explanation
//!
//! - **[`FXRequestAorB`]**: Used for queries involving Visa's daily indicative
//!   rates. This request is appropriate for card-based (`A`) and account-based
//!   (`B`) rates, providing options for setting a markup rate and acquirer
//!   details if required.
//!
//! - **[`FXRequestBankOrWallet`]**: Used for obtaining real-time or quoted
//!   rates for `BANK` and `WALLET` products. This request allows specifying
//!   either a source or destination amount, as well as an initiating party ID
//!   and optional quote ID requirement.
//!
//! - **[`FXResponseAorB`]**: Captures the exchange rate, destination amount,
//!   and optional markup rate details for requests using rate products `A` or
//!   `B`.
//!
//! - **[`FXResponseBankOrWallet`]**: Returns details about the conversion rate,
//!   amounts, and an optional quote ID with expiration information for rate
//!   products `BANK` and `WALLET`.
//!
//! ## Additional Information
//!
//! For further details, consult the Visa API documentation: [Visa Foreign
//! Exchange](https://developer.visa.com/capabilities/foreign_exchange).
//!

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Foreign Exchange Rates API request structure for rate product codes A or B.
///
/// This request structure is used to inquire about Visa's currency conversion
/// rates for a given currency pair using card-based (`A`) or account-based
/// (`B`) rates.
///
/// For more details, visit the [Visa Foreign Exchange API
/// Documentation](https://developer.visa.com/capabilities/foreign_exchange/reference).
#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
#[builder(build_fn(error = "crate::utils::BuilderError"))]
pub struct FXRequestAorB {
    /// ISO 4217 code of the source currency, as a 3-letter string (e.g.,
    /// "USD").
    ///
    /// Example: `"USD"`
    #[builder(setter(into))]
    pub source_currency_code: String,

    /// ISO 4217 code of the destination currency, as a 3-letter string (e.g.,
    /// "GBP").
    ///
    /// Example: `"GBP"`
    #[builder(setter(into))]
    pub destination_currency_code: String,

    /// The amount to convert from the source currency. This amount includes any
    /// markup.
    ///
    /// Format: A decimal with up to 2 digits after the decimal point. Example:
    /// `"100.55"`
    #[builder(setter(into))]
    pub source_amount: String,

    /// Optional FX markup rate to apply. Represents a percentage markup (e.g.,
    /// "0.07" for 0.07%).
    ///
    /// Example: `"0.07"`
    #[builder(default)]
    pub markup_rate: Option<String>,

    /// Additional information about the acquiring institution.
    #[builder(default)]
    pub acquirer_details: Option<AcquirerDetails>,
}

/// Foreign Exchange Rates API request structure for rate product codes BANK or
/// WALLET.
///
/// This request structure is used for real-time or quote-based rates, such as
/// account-based (`BANK`) or wallet-based (`WALLET`) rates.
///
/// For further details, consult the [Visa Foreign Exchange API
/// Documentation](https://developer.visa.com/capabilities/foreign_exchange/reference).
#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
#[builder(build_fn(error = "crate::utils::BuilderError"))]
pub struct FXRequestBankOrWallet {
    /// ISO 4217 code of the source currency, as a 3-letter string (e.g.,
    /// "USD").
    ///
    /// Example: `"USD"`
    #[builder(setter(into))]
    pub source_currency_code: String,

    /// ISO 4217 code of the destination currency, as a 3-letter string (e.g.,
    /// "GBP").
    ///
    /// Example: `"GBP"`
    #[builder(setter(into))]
    pub destination_currency_code: String,

    /// The source amount in the source currency, if known.
    ///
    /// Format: Decimal with up to 2 digits after the decimal point. Example:
    /// `100.55`
    #[builder(default)]
    pub source_amount: Option<f64>,

    /// The destination amount in the destination currency, if known.
    ///
    /// Format: Decimal with up to 2 digits after the decimal point. Example:
    /// `85.42`
    #[builder(default)]
    pub destination_amount: Option<f64>,

    /// ID assigned by Visa to identify the originating entity.
    ///
    /// Example: `1002`
    pub initiating_party_id: i64,

    /// Specifies whether a quote ID is required in the response for use in
    /// future transactions.
    ///
    /// Example: `true`
    #[builder(default)]
    pub quote_id_required: Option<bool>,
}

/// Details about the acquiring institution for requests using rate product
/// codes A or B.
#[derive(Clone, Debug, Serialize, Deserialize, Builder)]
#[builder(build_fn(error = "crate::utils::BuilderError"))]
pub struct AcquirerDetails {
    /// The Bank Identification Number (BIN) of the acquirer.
    ///
    /// Example: `408999`
    pub bin: u32,

    /// ISO 4217 numeric code of the settlement currency (e.g., "840" for USD).
    ///
    /// Example: `"840"`
    #[builder(setter(into))]
    pub settlement_currency_code: String,
}

/// Foreign Exchange Rates API response structure for rate product codes A or B.
///
/// This response structure provides conversion rate and amount details
/// following an FX inquiry for card-based (`A`) or account-based (`B`) rates.
///
/// Example response:
/// ```json
/// {
///     "conversion_rate": "0.07",
///     "destination_amount": "75.85",
///     "markup_rate_applied": "0.07",
///     "original_destn_amt_before_mark_up": "81.16"
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FXResponseAorB {
    /// Conversion rate applied to the transaction, excluding any markup.
    ///
    /// Example: `"0.07"`
    pub conversion_rate: String,

    /// Transaction amount in the destination currency, after conversion.
    ///
    /// Example: `"75.85"`
    pub destination_amount: String,

    /// The FX markup rate applied to the transaction, if any.
    ///
    /// Example: `"0.07"`
    pub markup_rate_applied: Option<String>,

    /// The destination amount before any markup was applied.
    ///
    /// Example: `"81.16"`
    pub original_destn_amt_before_mark_up: Option<String>,
}

/// Foreign Exchange Rates API response structure for rate product codes BANK or
/// WALLET.
///
/// This response provides a real-time or quote-based rate, and includes a quote
/// ID if requested.
///
/// Example response:
/// ```json
/// {
///     "conversion_rate": 0.07,
///     "source_amount": 100.55,
///     "destination_amount": 75.85,
///     "quote_id": 987654321,
///     "quote_id_expiry_datetime": "2024-01-08T10:22:15.529+00:00"
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FXResponseBankOrWallet {
    /// Conversion rate applied to convert the source amount to the destination
    /// amount.
    ///
    /// Example: `0.07`
    pub conversion_rate: f64,

    /// Source amount in the source currency.
    ///
    /// Example: `100.55`
    pub source_amount: Option<f64>,

    /// Destination amount in the destination currency.
    ///
    /// Example: `75.85`
    pub destination_amount: Option<f64>,

    /// Unique quote ID for the FX transaction, used in future transactions if
    /// needed.
    ///
    /// Example: `987654321`
    pub quote_id: Option<i64>,

    /// Expiration date and time for the quote ID, in ISO 8601 format.
    ///
    /// Example: `"2024-01-08T10:22:15.529+00:00"`
    pub quote_id_expiry_datetime: Option<String>,
}
