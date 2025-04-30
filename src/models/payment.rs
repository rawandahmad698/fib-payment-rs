use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct MonetaryValue {
    pub amount: f64,
    pub currency: String,
}

#[derive(Debug, Serialize)]
pub struct CreatePaymentRequest {
    #[serde(rename = "monetaryValue")]
    pub monetary_value: MonetaryValue,
    #[serde(rename = "statusCallbackUrl")]
    pub status_callback_url: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "refundableFor")]
    pub refundable_for: String,
}

#[derive(Debug, Deserialize)]
pub struct PaymentResponse {
    #[serde(rename = "paymentId")]
    pub payment_id: String,
    #[serde(rename = "readableCode")]
    pub readable_code: String,
    #[serde(rename = "qrCode")]
    pub qr_code: String,
    #[serde(rename = "validUntil")]
    pub valid_until: DateTime<Utc>,
    #[serde(rename = "personalAppLink")]
    pub personal_app_link: String,
    #[serde(rename = "businessAppLink")]
    pub business_app_link: String,
    #[serde(rename = "corporateAppLink")]
    pub corporate_app_link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaidBy {
    pub name: String,
    pub iban: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    Unpaid,
    Paid,
    RefundRequested,
    RefundInProgress,
    Refunded,
    Declined,
}

#[derive(Debug, Deserialize)]
pub struct PaymentStatusResponse {
    #[serde(rename = "paymentId")]
    pub payment_id: String,
    pub status: PaymentStatus,
    #[serde(rename = "paidAt")]
    pub paid_at: Option<DateTime<Utc>>,
    pub amount: MonetaryValue,
    #[serde(rename = "decliningReason")]
    pub declining_reason: Option<String>,
    #[serde(rename = "declinedAt")]
    pub declined_at: Option<DateTime<Utc>>,
    #[serde(rename = "paidBy")]
    pub paid_by: Option<PaidBy>,
}

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub title: String,
    pub detail: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    #[serde(rename = "traceId")]
    pub trace_id: String,
    pub errors: Vec<ApiError>,
} 