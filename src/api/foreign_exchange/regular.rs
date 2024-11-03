pub mod models;

#[double]
use crate::client::VisaClient;
use crate::{api::result::Result, client::utils::MLETrait};
use mockall_double::double;
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
    ///     .source_currency_code("USD")
    ///     .destination_currency_code("GBP")
    ///     .source_amount("100.55")
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
    ///     .source_currency_code("USD")
    ///     .destination_currency_code("GBP")
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::models::ApiLevel;
    use http::response::Builder as ResponseBuilder;

    const MOCK_URL: &str = "https://domain.test";

    fn setup_mock_execute_request(
        mock_client: &mut VisaClient<()>,
        url: &str,
        status: u16,
        body: &str,
    ) {
        let response = ResponseBuilder::new()
            .status(status)
            .body(body.to_string())
            .unwrap();

        let url_clone = url.to_string();
        mock_client
            .expect_execute_request()
            .withf(move |request| request.url().as_str() == url_clone)
            .returning(move |_| Ok(response.clone().into()));
    }

    fn setup_mock_get_config(mock_client: &mut VisaClient<()>, api_level: ApiLevel) {
        mock_client
            .expect_get_config()
            .return_const(crate::client::models::Config {
                api_level,
                ..Default::default()
            });
        mock_client
            .expect_get_base_url()
            .return_const(Url::parse(MOCK_URL).unwrap());
    }
    #[tokio::test]
    async fn test_foreign_exchange_get_a_or_b() {
        let mut mock_client = VisaClient::<()>::new();
        setup_mock_execute_request(
            &mut mock_client,
            format!("{}/forexrates/v2/foreignexchangerates", self::MOCK_URL).as_str(),
            200,
            r#"{
                "conversion_rate": "0.07",
                "destination_amount": "75.85",
                "markup_rate_applied": "0.07",
                "original_destn_amt_before_mark_up": "81.16"
            }"#,
        );
        setup_mock_get_config(&mut mock_client, ApiLevel::Sandbox);

        let forex = ForeignExchange::new(mock_client);
        let payload = FXRequestAorBBuilder::default()
            .source_currency_code("USD".to_string())
            .destination_currency_code("GBP".to_string())
            .source_amount("100.55".to_string())
            .build()
            .expect("Failed to build FXRequestAorB");

        let result = forex
            .get_a_or_b(payload)
            .await
            .expect("Failed to get response");

        assert_eq!(
            result,
            FXResponseAorB {
                conversion_rate: "0.07".to_string(),
                destination_amount: "75.85".to_string(),
                markup_rate_applied: Some("0.07".to_string()),
                original_destn_amt_before_mark_up: Some("81.16".to_string()),
            }
        );
    }

    #[tokio::test]
    async fn test_foreign_exchange_get_bank_or_wallet() {
        let mut mock_client = VisaClient::<()>::new();
        setup_mock_execute_request(
            &mut mock_client,
            format!("{}/forexrates/v2/foreignexchangerates", self::MOCK_URL).as_str(),
            200,
            r#"{
                "conversion_rate": 0.07,
                "source_amount": 100.55,
                "destination_amount": 75.85,
                "quote_id": 987654321,
                "quote_id_expiry_datetime": "2024-01-08T10:22:15.529+00:00"
            }"#,
        );
        setup_mock_get_config(&mut mock_client, ApiLevel::Sandbox);

        let forex = ForeignExchange::new(mock_client);
        let payload = FXRequestBankOrWalletBuilder::default()
            .source_currency_code("USD".to_string())
            .destination_currency_code("GBP".to_string())
            .initiating_party_id(1002)
            .source_amount(Some(100.55))
            .quote_id_required(Some(true))
            .build()
            .expect("Failed to build FXRequestBankOrWallet");

        let result = forex
            .get_bank_or_wallet(payload)
            .await
            .expect("Failed to get response");

        assert_eq!(
            result,
            FXResponseBankOrWallet {
                conversion_rate: 0.07,
                source_amount: Some(100.55),
                destination_amount: Some(75.85),
                quote_id: Some(987654321),
                quote_id_expiry_datetime: Some("2024-01-08T10:22:15.529+00:00".to_string()),
            }
        );
    }
}
