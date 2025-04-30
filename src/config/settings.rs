use std::env;
use url::Url;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct FibConfig {
    pub base_url: Url,
    pub client_id: String,
    pub client_secret: String,
    pub callback_url: Option<String>,
    pub refundable_for: String,
    pub currency: String,
}

impl FibConfig {
    pub fn from_env() -> Result<Self, Error> {
        dotenv::dotenv().ok();

        let base_url = env::var("FIB_BASE_URL")
            .map_err(|_| Error::Configuration("FIB_BASE_URL not set".to_string()))?;
        let base_url = Url::parse(&base_url)?;

        let client_id = env::var("FIB_CLIENT_ID")
            .map_err(|_| Error::Configuration("FIB_CLIENT_ID not set".to_string()))?;

        let client_secret = env::var("FIB_CLIENT_SECRET")
            .map_err(|_| Error::Configuration("FIB_CLIENT_SECRET not set".to_string()))?;

        Ok(Self {
            base_url,
            client_id,
            client_secret,
            callback_url: env::var("FIB_CALLBACK_URL").ok(),
            refundable_for: env::var("FIB_REFUNDABLE_FOR").unwrap_or_else(|_| "P7D".to_string()),
            currency: env::var("FIB_CURRENCY").unwrap_or_else(|_| "IQD".to_string()),
        })
    }

    pub fn auth_url(&self) -> Url {
        self.base_url.join("/auth/realms/fib-online-shop/protocol/openid-connect/token")
            .expect("Failed to construct auth URL")
    }

    pub fn api_url(&self) -> Url {
        self.base_url.join("protected/v1/")
            .expect("Failed to construct API URL")
    }
} 