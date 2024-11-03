pub mod models;

use crate::{
    api::result::Result,
    client::{utils::MLETrait, VisaClient},
};
use models::*;
use reqwest::{Method, Request};
use serde_json::json;
use url::Url;

/// [`ForeignExchange`] struct provides methods to interact with the Visa
/// Foreign Exchange API.
///
/// This struct allows you to create requests to fetch foreign exchange rates
/// using different payloads. It requires a `VisaClient` instance to be
/// initialized.
///
/// # Example
///
/// ```
/// let client = VisaClient::new(api_key, secret_key);
/// let forex = ForeignExchange::new(client);
/// ```
///
/// Guide:
/// - <https://developer.visa.com/capabilities/foreign_exchange>
#[derive(Clone)]
pub struct ForeignExchange<MLE>
where
    MLE: MLETrait,
{
    client: VisaClient<MLE>,
    url: Url,
}

impl<MLE> ForeignExchange<MLE>
where
    MLE: MLETrait,
{
    const URL: &'static str = "/forexrates/v2/foreignexchangerates";

    /// Creates a new instance of `ForeignExchange`.
    ///
    /// # Arguments
    ///
    /// - `client`: A `VisaClient` instance. Does not need to have Message Level
    ///   Encryption enabled.
    ///
    /// # Example
    ///
    /// ```
    /// let client = VisaClient::new(api_key, secret_key);
    /// let forex = ForeignExchange::new(client);
    /// ```
    pub fn new(client: VisaClient<MLE>) -> Self {
        let url = client.get_base_url().join(Self::URL).unwrap();
        ForeignExchange { client, url }
    }

    /// Fetches foreign exchange rates using `FXRequestAorB` payload.
    ///
    /// This function returns indicative daily rates for transactions with card
    /// or bank-account as the payment instrument.
    ///
    /// # Arguments
    ///
    /// - `payload`: An instance of `FXRequestAorB`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let payload = FXRequestAorBBuilder::default()
    ///     .source_currency_code("USD".to_string())
    ///     .destination_currency_code("GBP".to_string())
    ///     .source_amount("100.55".to_string())
    ///     .build()
    ///     .expect("Failed to build FXRequestAorB");
    /// let response = forex.get_a_or_b(payload).await.expect("Failed to get response");
    /// println!("{:?}", response);
    /// ```
    pub async fn get_a_or_b(&self, payload: FXRequestAorB) -> Result<FXResponseAorB> {
        let mut request = Request::new(Method::GET, self.url.clone());
        request
            .body_mut()
            .replace(json!(payload).to_string().into());
        let response = self.client.execute_request(request).await?;
        Ok(response.json::<FXResponseAorB>().await?)
    }

    /// Fetches foreign exchange rates using `FXRequestBankOrWallet` payload.
    ///
    /// This function returns real-time rates for transactions with bank-account
    /// or wallet as the payment instrument.
    ///
    /// # Arguments
    ///
    /// - `payload`: An instance of `FXRequestBankOrWallet`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let payload = FXRequestBankOrWalletBuilder::default()
    ///     .source_currency_code("USD".to_string())
    ///     .destination_currency_code("GBP".to_string())
    ///     .initiating_party_id(1002)
    ///     .source_amount(Some(100.55))
    ///     .quote_id_required(Some(true))
    ///     .build()
    ///     .expect("Failed to build FXRequestBankOrWallet");
    /// let response = forex.get_bank_or_wallet(payload).await.expect("Failed to get response");
    /// println!("{:?}", response);
    /// ```
    pub async fn get_bank_or_wallet(
        &self,
        payload: FXRequestBankOrWallet,
    ) -> Result<FXResponseBankOrWallet> {
        let mut request = Request::new(Method::GET, self.url.clone());
        request
            .body_mut()
            .replace(json!(payload).to_string().into());
        let response = self.client.execute_request(request).await?;
        Ok(response.json::<FXResponseBankOrWallet>().await?)
    }
}
