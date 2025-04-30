use crate::{config::FibConfig, error::Error, models::{CreatePaymentRequest, MonetaryValue, PaymentResponse},
            repository::{FibPaymentRepository, PaymentRepository}, PaymentStatusResponse};

pub struct FibClient {
    repository: FibPaymentRepository,
}

impl FibClient {
    pub fn new(config: FibConfig) -> Self {
        Self {
            repository: FibPaymentRepository::new(config),
        }
    }

    pub async fn create_payment(
        &self,
        amount: f64,
        currency: Option<String>,
        callback_url: Option<String>,
        description: Option<String>,
        refundable_for: Option<String>,
    ) -> Result<PaymentResponse, Error> {
        let config = &self.repository.config;
        
        let request = CreatePaymentRequest {
            monetary_value: MonetaryValue {
                amount,
                currency: currency.unwrap_or_else(|| config.currency.clone()),
            },
            status_callback_url: callback_url.or_else(|| config.callback_url.clone()),
            description,
            refundable_for: refundable_for.unwrap_or_else(|| config.refundable_for.clone()),
        };

        self.repository.create_payment(request).await
    }

    pub async fn get_payment_status(&self, payment_id: &str) -> Result<PaymentStatusResponse, Error> {
        self.repository.get_payment_status(payment_id).await
    }

    pub async fn refund_payment(&self, payment_id: &str) -> Result<PaymentResponse, Error> {
        self.repository.refund_payment(payment_id).await
    }

    pub async fn cancel_payment(&self, payment_id: &str) -> Result<PaymentResponse, Error> {
        self.repository.cancel_payment(payment_id).await
    }
} 