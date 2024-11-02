use lazy_static::lazy_static;
use url::Url;

lazy_static! {
    pub static ref VISA_DOMAIN_SANDBOX: Url = Url::parse("https://sandbox.api.visa.com").unwrap();
    pub static ref VISA_DOMAIN_CERTIFICATION: Url =
        Url::parse("https://cert.api.visa.com").unwrap();
    pub static ref VISA_DOMAIN_PRODUCTION: Url = Url::parse("https://api.visa.com").unwrap();
}
