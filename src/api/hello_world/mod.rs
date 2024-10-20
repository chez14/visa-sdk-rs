use crate::client::{api_level::ApiLevel, state};
use mockall_double::double;
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
        // TODO: handle the unwraps
        let client = self.client.build_reqwest();
        let request = client.get(self.url.as_str());
        let response = self.client.apply_auth(request).send().await.unwrap();
        response.json::<serde_json::Value>().await.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{apply_mock, client::config::Config};
    use serde_json::json;

    #[tokio::test]
    async fn test_hello_world_get_production() {
        let mut mock_client: VisaClient<()> = VisaClient::default();
        let mut server = mockito::Server::new_async().await;
        let mock_reqwest_client = reqwest::Client::new();
        let config = Config {
          api_level: ApiLevel::Production,
      };

        mock_client
            .expect_build_reqwest()
            .return_const(mock_reqwest_client.clone());
        mock_client.expect_apply_auth().returning(|req| req);
        mock_client.expect_get_config().return_const(config);

        let mut hello_world = HelloWorld::new(mock_client);
        apply_mock!(hello_world, server);

        let mock_response = json!({
            "message": "Hello, World!"
        });

        let _m = server
            .mock("GET", "/helloworld")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let response = hello_world.get().await;
        assert_eq!(response, mock_response);
    }

    #[tokio::test]
    async fn test_hello_world_get_certification() {
        let mut mock_client: VisaClient<()> = VisaClient::default();
        let mut server = mockito::Server::new_async().await;
        let mock_reqwest_client = reqwest::Client::new();
        let config = Config {
            api_level: ApiLevel::Certification,
        };

        mock_client
            .expect_build_reqwest()
            .return_const(mock_reqwest_client.clone());
        mock_client.expect_apply_auth().returning(|req| req);
        mock_client.expect_get_config().return_const(config);

        let mut hello_world = HelloWorld::new(mock_client);
        apply_mock!(hello_world, server);

        let mock_response = json!({
            "message": "Hello, World!"
        });

        let _m = server
            .mock("GET", "/vdp/helloworld")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let response = hello_world.get().await;
        assert_eq!(response, mock_response);
    }
}
