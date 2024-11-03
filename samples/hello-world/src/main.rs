use std::env;
use visa_sdk::{
    api::hello_world::HelloWorld,
    client::{models::MutualTlsBuilder, VisaClientBuilder},
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
        .set_mutual_tls(
            MutualTlsBuilder::default()
                .user_id(user_id)
                .password(password)
                .cert(pfx_bytes)
                .cert_key(cert_key)
                .build()
                .unwrap(),
        )
        .build();

    let hello = HelloWorld::new(client);

    let response = hello.get().await;
    println!("{:?}", response);
    Ok(())
}
