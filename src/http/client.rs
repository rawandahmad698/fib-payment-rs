use backoff::{ExponentialBackoff, ExponentialBackoffBuilder};
use reqwest::{Client, Response, StatusCode};
use std::time::Duration;
use crate::error::Error;
use crate::models::ApiErrorResponse;

pub struct HttpClient {
    client: Client,
    backoff: ExponentialBackoff,
}

impl Default for HttpClient {
    fn default() -> Self {
        let backoff = ExponentialBackoffBuilder::new()
            .with_initial_interval(Duration::from_millis(100))
            .with_max_interval(Duration::from_secs(10))
            .with_multiplier(2.0)
            .with_max_elapsed_time(Some(Duration::from_secs(30)))
            .build();

        Self {
            client: Client::new(),
            backoff,
        }
    }
}

impl HttpClient {
    async fn handle_error_response(&self, response: Response) -> Result<Error, Error> {
        let status = response.status();
        match response.json::<ApiErrorResponse>().await {
            Ok(error_response) => {
                let error_code = error_response.errors.first()
                    .map(|e| e.code.clone())
                    .unwrap_or_else(|| "UNKNOWN_ERROR".to_string());
                
                let error_message = error_response.errors.first()
                    .and_then(|e| {
                        if e.detail.is_empty() {
                            None
                        } else {
                            Some(e.detail.clone())
                        }
                    })
                    .unwrap_or_else(|| format!("API error occurred with code: {}", error_code));

                Ok(Error::ApiError {
                    status_code: status.as_u16(),
                    code: error_code,
                    message: error_message,
                    trace_id: Some(error_response.trace_id),
                })
            },
            Err(_) => {
                Ok(Error::ApiError {
                    status_code: status.as_u16(),
                    code: "UNKNOWN_ERROR".to_string(),
                    message: "Failed to parse error response".to_string(),
                    trace_id: None,
                })
            }
        }
    }

    pub async fn request_with_retry<F, Fut, T>(&self, operation: F) -> Result<T, Error>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, Error>>,
    {
        backoff::future::retry(self.backoff.clone(), || async {
            match operation().await {
                Ok(value) => Ok(value),
                Err(e) => match &e {
                    Error::Network(err) if err.is_timeout() || err.is_connect() => {
                        Err(backoff::Error::transient(e))
                    }
                    _ => Err(backoff::Error::permanent(e)),
                },
            }
        })
        .await
    }

    pub async fn send(&self, request: reqwest::Request) -> Result<Response, Error> {
        self.request_with_retry(|| async {
            let resp = self.client.execute(request.try_clone().unwrap()).await?;

            match resp.status() {
                StatusCode::UNAUTHORIZED => {
                    Err(Error::ApiError {
                        status_code: 401,
                        code: "UNAUTHORIZED".to_string(),
                        message: "Invalid or expired token".to_string(),
                        trace_id: None,
                    })
                }
                status if !status.is_success() => {
                    Err(self.handle_error_response(resp).await?)
                }
                _ => Ok(resp),
            }
        })
        .await
    }
} 