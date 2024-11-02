pub mod models;

use crate::client::{utils::MLETrait, VisaClient};
use models::*;
use reqwest::{Method, Request};
use serde_json::json;
use url::Url;

/// Foreign Exchange API
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

    pub fn new(client: VisaClient<MLE>) -> Self {
        let url = client.get_base_url().join(Self::URL).unwrap();
        ForeignExchange { client, url }
    }

    pub async fn get_a_or_b(&self, payload: FXRequestAorB) -> FXResponseAorB {
        let mut request = Request::new(Method::GET, self.url.clone());
        request
            .body_mut()
            .replace(json!(payload).to_string().into());
        let response = self.client.execute_request(request).await.unwrap();
        response.json::<FXResponseAorB>().await.unwrap()
    }

    pub async fn get_bank_or_wallet(
        &self,
        payload: FXRequestBankOrWallet,
    ) -> FXResponseBankOrWallet {
        let mut request = Request::new(Method::GET, self.url.clone());
        request
            .body_mut()
            .replace(json!(payload).to_string().into());
        let response = self.client.execute_request(request).await.unwrap();
        response.json::<FXResponseBankOrWallet>().await.unwrap()
    }
}
