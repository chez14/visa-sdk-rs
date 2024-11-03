use derive_builder::Builder;

#[allow(dead_code)] // TODO: Remove this.
#[derive(Default, Clone, Builder)]
#[builder(build_fn(error = "crate::utils::BuilderError"))]
pub struct MessageLevelEncryption {
    #[builder(setter(into))]
    pub(crate) client_private_key: String,

    #[builder(setter(into))]
    pub(crate) client_private_key_pass: Option<String>,

    #[builder(setter(into))]
    pub(crate) server_public_key: String,
}
