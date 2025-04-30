use async_trait::async_trait;
use reqwest::header::{
    HeaderMap,
    HeaderValue,
    AUTHORIZATION,
    CONTENT_TYPE,
    USER_AGENT
};
use tokio::sync::Mutex;

use crate::config::FibConfig;
use crate::error::Error;
use crate::http::HttpClient;
use crate::models::{CreatePaymentRequest, PaymentResponse, PaymentStatusResponse, TokenResponse};
use std::collections::HashMap;

#[async_trait]
pub trait PaymentRepository {
    async fn create_payment(&self, request: CreatePaymentRequest) -> Result<PaymentResponse, Error>;
    async fn get_payment_status(&self, payment_id: &str) -> Result<PaymentStatusResponse, Error>;
    async fn refund_payment(&self, payment_id: &str) -> Result<PaymentResponse, Error>;
    async fn cancel_payment(&self, payment_id: &str) -> Result<PaymentResponse, Error>;
}

pub struct FibPaymentRepository {
    pub(crate) config: FibConfig,
    http_client: HttpClient,
    token: Mutex<Option<String>>,
}

impl FibPaymentRepository {
    pub fn new(config: FibConfig) -> Self {
        Self {
            config,
            http_client: HttpClient::default(),
            token: Mutex::new(None),
        }
    }

    async fn get_token(&self) -> Result<String, Error> {
        let mut token_guard = self.token.lock().await;
        if let Some(token) = token_guard.as_ref() {
            return Ok(token.clone());
        }

        // Prepare form data (x-www-form-urlencoded)
        let mut form = HashMap::new();
        form.insert("grant_type", "client_credentials");
        form.insert("client_id", &self.config.client_id);
        form.insert("client_secret", &self.config.client_secret);

        let body = serde_urlencoded::to_string(&form)
            .map_err(|e| Error::Configuration(format!("Failed to encode form: {}", e)))?;

        // Build headers
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));

        // Construct the request
        let request = reqwest::Request::new(reqwest::Method::POST, self.config.auth_url());
        let mut request = request;
        *request.headers_mut() = headers;
        *request.body_mut() = Some(body.into());

        // Send request using custom HttpClient
        let response = self.http_client.send(request).await?;

        // Parse and return the token
        let token_response: TokenResponse = response.json().await?;
        *token_guard = Some(token_response.access_token.clone());
        Ok(token_response.access_token)
    }

    async fn create_headers(&self) -> Result<HeaderMap, Error> {
        let token = self.get_token().await?;
        let mut headers = HeaderMap::new();
        
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("FibPaymentsSDK-Rust"));

        Ok(headers)
    }
}

#[async_trait]
impl PaymentRepository for FibPaymentRepository {
    async fn create_payment(&self, request: CreatePaymentRequest) -> Result<PaymentResponse, Error> {
        // Headers
        let mut headers = self.create_headers().await?;
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Serialize JSON
        let body = serde_json::to_vec(&request)?;
        let url = self.config.api_url().join("payments")?;

        let mut req = reqwest::Request::new(reqwest::Method::POST, url);
        *req.headers_mut() = headers;
        *req.body_mut() = Some(body.into());

        // Send and parse
        let response = self.http_client.send(req).await?;
        let payment = response.json().await?;

        Ok(payment)
    }

    async fn get_payment_status(&self, payment_id: &str) -> Result<PaymentStatusResponse, Error> {
        let headers = self.create_headers().await?;
        let url = self.config.api_url()
            .join(&format!("payments/{}/status", payment_id))?;

        let mut req = reqwest::Request::new(
            reqwest::Method::GET,
            url,
        );
        *req.headers_mut() = headers;

        let response = self.http_client.send(req).await?;
        let status = response.json().await?;
        Ok(status)
    }

    async fn refund_payment(&self, payment_id: &str) -> Result<PaymentResponse, Error> {
        let headers = self.create_headers().await?;
        let url = self.config.api_url()
            .join(&format!("payments/{}/refund", payment_id))?;

        let mut req = reqwest::Request::new(
            reqwest::Method::POST,
            url,
        );
        *req.headers_mut() = headers;

        let response = self.http_client.send(req).await?;
        let payment = response.json().await?;
        Ok(payment)
    }

    async fn cancel_payment(&self, payment_id: &str) -> Result<PaymentResponse, Error> {
        let headers = self.create_headers().await?;
        let url = self.config.api_url()
            .join(&format!("payments/{}/cancel", payment_id))?;

        let mut req = reqwest::Request::new(
            reqwest::Method::POST,
            url,
        );
        *req.headers_mut() = headers;

        let response = self.http_client.send(req).await?;
        let payment = response.json().await?;
        Ok(payment)
    }
} 