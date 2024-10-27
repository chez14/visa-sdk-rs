use crate::client::{api_level::ApiLevel, state};
use mockall_double::double;
use reqwest::{Method, Request};
use url::Url;

#[double]
use crate::client::VisaClient;

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
    MLE: state::MessageLevelEncryptionState + Clone,
{
    client: VisaClient<MLE>,
    url: Url,
}

impl<MLE> HelloWorld<MLE>
where
    MLE: state::MessageLevelEncryptionState + Clone,
{
    pub fn new(client: VisaClient<MLE>) -> Self {
        let url = Url::parse(match client.get_config().api_level {
            ApiLevel::Sandbox => "https://sandbox.api.visa.com/vdp/helloworld",
            ApiLevel::Certification => "https://cert.api.visa.com/vdp/helloworld",
            ApiLevel::Production => "https://api.visa.com/helloworld",
        })
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
    use serde_json::json;
    use http::response::Builder as ResponseBuilder;

    #[tokio::test]
    async fn test_hello_world_get_sandbox() {
        let mut mock_client = VisaClient::<()>::new();
        let response = ResponseBuilder::new()
            .status(200)
            .body(r#"{"message": "Hello, World!"}"#)
            .unwrap();

        mock_client
            .expect_execute_request()
            .withf(|request| {
                request.url().as_str() == "https://sandbox.api.visa.com/vdp/helloworld"
            })
            .returning(move |_| Ok(response.clone().into()));

        mock_client
            .expect_get_config()
            .return_const(crate::client::config::Config {
                api_level: ApiLevel::Sandbox,
                ..Default::default()
            });

        let hello_world = HelloWorld::new(mock_client);
        let result = hello_world.get().await;

        assert_eq!(result, json!({"message": "Hello, World!"}));
    }

    #[tokio::test]
    async fn test_hello_world_get_certification() {
        let mut mock_client = VisaClient::<()>::new();
        let response = ResponseBuilder::new()
            .status(200)
            .body(r#"{"message": "Hello, World!"}"#)
            .unwrap();

        mock_client
            .expect_execute_request()
            .withf(|request| request.url().as_str() == "https://cert.api.visa.com/vdp/helloworld")
            .returning(move |_| Ok(response.clone().into()));

        mock_client
            .expect_get_config()
            .return_const(crate::client::config::Config {
                api_level: ApiLevel::Certification,
                ..Default::default()
            });

        let hello_world = HelloWorld::new(mock_client);
        let result = hello_world.get().await;

        assert_eq!(result, json!({"message": "Hello, World!"}));
    }

    #[tokio::test]
    async fn test_hello_world_get_production() {
        let mut mock_client = VisaClient::<()>::new();
        let response = ResponseBuilder::new()
            .status(200)
            .body(r#"{"message": "Hello, World!"}"#)
            .unwrap();

        mock_client
            .expect_execute_request()
            .withf(|request| request.url().as_str() == "https://api.visa.com/helloworld")
            .returning(move |_| Ok(response.clone().into()));

        mock_client
            .expect_get_config()
            .return_const(crate::client::config::Config {
                api_level: ApiLevel::Production,
                ..Default::default()
            });

        let hello_world = HelloWorld::new(mock_client);
        let result = hello_world.get().await;

        assert_eq!(result, json!({"message": "Hello, World!"}));
    }
}
