
#[derive(Default, Clone)]
pub struct MessageLevelEncryption {
    pub client_private_key: String,
    pub client_private_key_pass: Option<String>,
    pub server_public_key: String,
}
