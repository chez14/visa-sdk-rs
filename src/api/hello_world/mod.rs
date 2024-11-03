#[double]
use crate::client::VisaClient;
use crate::client::{models::ApiLevel, utils::MLETrait};
use mockall_double::double;
use reqwest::{Method, Request};
use url::Url;

/// Visa API for the HelloWorld endpoint. This API is used to test the
/// connection to the Visa API in certification and production environments.
///
/// See
/// <https://developer.visa.com/pages/working-with-visa-apis/visa-developer-quick-start-guide#section10>
///
/// ## Example
///
/// ```no_run
/// use visa_sdk::client::state::MutualTls;
/// use visa_sdk::api::hello_world;
///
/// let mtls = MutualTls {
///    user_id: "application-id-from-visa-sama".to_string(),
///   password: "super-secret-password-from-visa-sama".to_string()
///       cert: "--- BEGIN ...".to_string(),
///   cert_key: Some("super-secret-key".to_string()),
///  ca_bundle: "your_ca_bundle".to_string(),
/// };
///
/// let client = VisaClient::builder()
///    .set_mutual_tls(mtls)
///    .build();
///
/// let hello_world = HelloWorld::new(client, false);
/// let response = hello_world.get().await;
/// println!("{:?}", response);
/// ```
#[derive(Clone)]
pub struct HelloWorld<MLE>
where
    MLE: MLETrait,
{
    client: VisaClient<MLE>,
    url: Url,
}

impl<MLE> HelloWorld<MLE>
where
    MLE: MLETrait,
{
    pub fn new(client: VisaClient<MLE>) -> Self {
        let base_url = client.get_base_url();
        let url = match client.get_config().api_level {
            ApiLevel::Production => base_url.join("/helloworld"),
            _ => base_url.join("/vdp/helloworld"),
        }
        .unwrap();
        HelloWorld { client, url }
    }

    pub async fn get(&self) -> serde_json::Value {
        let request = Request::new(Method::GET, self.url.clone());
        // TODO: handle the unwraps
        let response = self.client.execute_request(request).await.unwrap();
        response.json::<serde_json::Value>().await.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::response::Builder as ResponseBuilder;
    use serde_json::json;

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
    async fn test_hello_world_get_sandbox() {
        let mut mock_client = VisaClient::<()>::new();
        setup_mock_execute_request(
            &mut mock_client,
            format!("{}/vdp/helloworld", self::MOCK_URL).as_str(),
            200,
            r#"{"message": "Hello, World!"}"#,
        );
        setup_mock_get_config(&mut mock_client, ApiLevel::Sandbox);

        let hello_world = HelloWorld::new(mock_client);
        let result = hello_world.get().await;

        assert_eq!(result, json!({"message": "Hello, World!"}));
    }

    #[tokio::test]
    async fn test_hello_world_get_certification() {
        let mut mock_client = VisaClient::<()>::new();
        setup_mock_execute_request(
            &mut mock_client,
            format!("{}/vdp/helloworld", self::MOCK_URL).as_str(),
            200,
            r#"{"message": "Hello, World!"}"#,
        );
        setup_mock_get_config(&mut mock_client, ApiLevel::Certification);

        let hello_world = HelloWorld::new(mock_client);
        let result = hello_world.get().await;

        assert_eq!(result, json!({"message": "Hello, World!"}));
    }

    #[tokio::test]
    async fn test_hello_world_get_production() {
        let mut mock_client = VisaClient::<()>::new();
        setup_mock_execute_request(
            &mut mock_client,
            format!("{}/helloworld", self::MOCK_URL).as_str(),
            200,
            r#"{"message": "Hello, World!"}"#,
        );
        setup_mock_get_config(&mut mock_client, ApiLevel::Production);

        let hello_world = HelloWorld::new(mock_client);
        let result = hello_world.get().await;

        assert_eq!(result, json!({"message": "Hello, World!"}));
    }
}
