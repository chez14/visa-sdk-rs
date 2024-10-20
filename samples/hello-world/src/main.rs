use std::env;
use visa_sdk::{
    api::hello_world::HelloWorld,
    client::{builder::VisaClientBuilder, mutual_tls::MutualTls},
};

#[tokio::main]
async fn main() -> Result<(), ()> {
    dotenvy::dotenv().unwrap();
    let user_id = env::var("VISA_USER_ID").unwrap();
    let password = env::var("VISA_PASSWORD").unwrap();
    let cert_path = env::var("VISA_CERT").unwrap();
    let cert_key = env::var("VISA_CERT_PASSWORD").map_or(None, |v| Some(v));
    let pfx_bytes = std::fs::read(cert_path).unwrap();

    let client = VisaClientBuilder::new()
        .set_mutual_tls(MutualTls {
            user_id,
            password,
            cert: pfx_bytes.to_vec(),
            cert_key,
        })
        .build();

    let hello = HelloWorld::new(client);

    let response = hello.get().await;
    println!("{:?}", response);
    Ok(())
}
